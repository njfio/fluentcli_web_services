use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::api_key::{ApiKey, NewApiKey};
use crate::schema::api_keys;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub struct ApiKeyService;

impl ApiKeyService {
    pub fn create_api_key(
        pool: &DbPool,
        user_id: Uuid,
        key_value: String,
        description: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey, AppError> {
        let hashed_key =
            hash(&key_value, DEFAULT_COST).map_err(|_| AppError::InternalServerError)?;
        let new_api_key = NewApiKey {
            user_id,
            key_value: hashed_key,
            description,
            expires_at,
        };

        let mut conn = pool.get()?;
        let api_key = diesel::insert_into(api_keys::table)
            .values(&new_api_key)
            .get_result(&mut conn)
            .map_err(AppError::from)?;

        Ok(ApiKey {
            key_value,
            ..api_key
        })
    }

    pub fn get_api_key_by_id(pool: &DbPool, id: Uuid) -> Result<Option<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        api_keys::table
            .find(id)
            .first(&mut conn)
            .optional()
            .map_err(AppError::from)
    }

    pub fn update_api_key(
        pool: &DbPool,
        id: Uuid,
        description: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ApiKey, AppError> {
        let mut conn = pool.get()?;
        diesel::update(api_keys::table.find(id))
            .set((
                api_keys::description.eq(description),
                api_keys::expires_at.eq(expires_at),
                api_keys::updated_at.eq(Utc::now()),
            ))
            .get_result(&mut conn)
            .map_err(AppError::from)
    }

    pub fn delete_api_key(pool: &DbPool, id: Uuid) -> Result<bool, AppError> {
        let mut conn = pool.get()?;
        let affected_rows = diesel::delete(api_keys::table.find(id)).execute(&mut conn)?;
        Ok(affected_rows > 0)
    }

    pub fn list_api_keys_for_user(pool: &DbPool, user_id: Uuid) -> Result<Vec<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        api_keys::table
            .filter(api_keys::user_id.eq(user_id))
            .load(&mut conn)
            .map_err(AppError::from)
    }

    pub fn verify_api_key(pool: &DbPool, key_value: &str) -> Result<Option<ApiKey>, AppError> {
        let mut conn = pool.get()?;
        let api_keys: Vec<ApiKey> = api_keys::table.load(&mut conn).map_err(AppError::from)?;

        for api_key in api_keys {
            if verify(key_value, &api_key.key_value).map_err(|_| AppError::InternalServerError)? {
                return Ok(Some(api_key));
            }
        }

        Ok(None)
    }
}
