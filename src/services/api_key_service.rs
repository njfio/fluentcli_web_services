use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::api_key::{ApiKey, NewApiKey};
use crate::schema::{api_keys, user_llm_configs};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_padding::Pkcs7;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use hex;
use log::{debug, error, warn};
use rand::Rng;
use uuid::Uuid;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct ApiKeyService;

impl ApiKeyService {
    fn get_encryption_key() -> Result<[u8; 32], AppError> {
        let encryption_key =
            std::env::var("ENCRYPTION_KEY").map_err(|_| AppError::InternalServerError)?;
        debug!("Encryption key length: {}", encryption_key.len());
        hex::decode(encryption_key)
            .map_err(|_| AppError::InternalServerError)?
            .try_into()
            .map_err(|_| AppError::InternalServerError)
    }

    fn encrypt_key(key: &str) -> Result<String, AppError> {
        let encryption_key = Self::get_encryption_key()?;
        let iv: [u8; 16] = rand::thread_rng().gen();
        let cipher = Aes256Cbc::new_from_slices(&encryption_key, &iv)
            .map_err(|_| AppError::InternalServerError)?;

        let ciphertext = cipher.encrypt_vec(key.as_bytes());
        let mut result = iv.to_vec();
        result.extend_from_slice(&ciphertext);
        let encrypted = hex::encode(result);
        debug!("Encrypted key length: {}", encrypted.len());
        Ok(encrypted)
    }

    fn decrypt_key(encrypted_key: &str) -> Result<String, AppError> {
        let encryption_key = Self::get_encryption_key()?;
        let encrypted_data =
            hex::decode(encrypted_key).map_err(|_| AppError::InternalServerError)?;

        if encrypted_data.len() < 16 {
            return Err(AppError::InternalServerError);
        }

        let (iv, ciphertext) = encrypted_data.split_at(16);
        let cipher = Aes256Cbc::new_from_slices(&encryption_key, iv)
            .map_err(|_| AppError::InternalServerError)?;

        let decrypted_data = cipher
            .decrypt_vec(ciphertext)
            .map_err(|_| AppError::InternalServerError)?;

        let decrypted =
            String::from_utf8(decrypted_data).map_err(|_| AppError::InternalServerError)?;
        debug!("Decrypted key length: {}", decrypted.len());
        Ok(decrypted)
    }

    pub fn create_api_key(
        pool: &DbPool,
        user_id: Uuid,
        key_value: String,
        description: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey, AppError> {
        debug!(
            "Creating new API key. Original key length: {}",
            key_value.len()
        );
        let encrypted_key = Self::encrypt_key(&key_value)?;
        debug!("Encrypted key length: {}", encrypted_key.len());
        let new_api_key = NewApiKey {
            user_id,
            key_value: encrypted_key,
            description,
            expires_at,
        };

        let mut conn = pool.get()?;
        let mut api_key = diesel::insert_into(api_keys::table)
            .values(&new_api_key)
            .get_result::<ApiKey>(&mut conn)
            .map_err(AppError::from)?;

        // Return the unencrypted key
        api_key.key_value = key_value;
        debug!(
            "API key created successfully. Returned key length: {}",
            api_key.key_value.len()
        );
        Ok(api_key)
    }

    pub fn get_api_key_by_id(pool: &DbPool, id: Uuid) -> Result<Option<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        debug!("Retrieving API key with ID: {}", id);
        let api_key = api_keys::table
            .find(id)
            .first::<ApiKey>(&mut conn)
            .optional()
            .map_err(AppError::from)?;

        match api_key {
            Some(mut api_key) => {
                debug!(
                    "API key found. Encrypted key length: {}",
                    api_key.key_value.len()
                );
                api_key.key_value = Self::decrypt_key(&api_key.key_value)?;
                debug!(
                    "API key decrypted. Decrypted key length: {}",
                    api_key.key_value.len()
                );
                Ok(Some(api_key))
            }
            None => {
                warn!("API key not found for ID: {}", id);
                Ok(None)
            }
        }
    }
    pub fn update_api_key(
        pool: &DbPool,
        id: Uuid,
        description: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey, AppError> {
        debug!("Updating API key. Note: key_value is not expected or used in updates.");
        let mut conn = pool.get()?;
        let updated_api_key = diesel::update(api_keys::table.find(id))
            .set((
                api_keys::description.eq(description),
                api_keys::expires_at.eq(expires_at),
                api_keys::updated_at.eq(Utc::now()),
            ))
            .get_result::<ApiKey>(&mut conn)
            .map_err(AppError::from)?;

        // Decrypt the key before returning
        let mut decrypted_api_key = updated_api_key;
        decrypted_api_key.key_value = Self::decrypt_key(&decrypted_api_key.key_value)?;
        debug!(
            "Updated API key. Decrypted key length: {}",
            decrypted_api_key.key_value.len()
        );
        Ok(decrypted_api_key)
    }
    pub fn delete_api_key(pool: &DbPool, id: Uuid) -> Result<bool, AppError> {
        let mut conn = pool.get()?;
        conn.transaction(|conn| {
            // First, delete associated user_llm_configs
            let deleted_configs =
                diesel::delete(user_llm_configs::table.filter(user_llm_configs::api_key_id.eq(id)))
                    .execute(conn)?;
            debug!("Deleted {} associated user_llm_configs", deleted_configs);

            // Then, delete the API key
            let affected_rows = diesel::delete(api_keys::table.find(id)).execute(conn)?;
            debug!("Deleted API key. Affected rows: {}", affected_rows);
            Ok(affected_rows > 0)
        })
    }

    pub fn list_api_keys_for_user(pool: &DbPool, user_id: Uuid) -> Result<Vec<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        let encrypted_keys = api_keys::table
            .filter(api_keys::user_id.eq(user_id))
            .load::<ApiKey>(&mut conn)
            .map_err(AppError::from)?;

        // Decrypt the keys before returning
        let mut decrypted_keys = Vec::new();
        for mut key in encrypted_keys {
            debug!("Decrypting key. Encrypted length: {}", key.key_value.len());
            key.key_value = Self::decrypt_key(&key.key_value)?;
            debug!("Key decrypted. Decrypted length: {}", key.key_value.len());
            decrypted_keys.push(key);
        }
        debug!("Listed {} API keys for user", decrypted_keys.len());
        Ok(decrypted_keys)
    }

    pub fn verify_api_key(pool: &DbPool, key_value: &str) -> Result<Option<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        let api_keys: Vec<ApiKey> = api_keys::table.load(&mut conn).map_err(AppError::from)?;

        for mut api_key in api_keys {
            let decrypted_key = Self::decrypt_key(&api_key.key_value)?;
            if decrypted_key == key_value {
                api_key.key_value = decrypted_key;
                debug!("API key verified successfully");
                return Ok(Some(api_key));
            }
        }

        debug!("API key verification failed");
        Ok(None)
    }
}
