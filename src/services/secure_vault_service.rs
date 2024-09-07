use crate::db::DbPool;
use crate::error::AppError;
use crate::models::secure_vault::{SecureVault, NewSecureVault, UpdateSecureVault};
use diesel::prelude::*;
use uuid::Uuid;

pub struct SecureVaultService;

impl SecureVaultService {
    pub fn create_secure_vault(pool: &DbPool, new_secure_vault: NewSecureVault) -> Result<SecureVault, AppError> {
        use crate::schema::secure_vaults::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new secure vault into database: {:?}", new_secure_vault);
        match diesel::insert_into(secure_vaults)
            .values(&new_secure_vault)
            .get_result(conn) {
            Ok(secure_vault) => {
                log::info!("Secure vault inserted into database: {:?}", secure_vault);
                Ok(secure_vault)
            },
            Err(e) => {
                log::error!("Error inserting secure vault into database: {:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn list_secure_vaults(pool: &DbPool, user_id: Uuid) -> Result<Vec<SecureVault>, AppError> {
        use crate::schema::secure_vaults::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Listing secure vaults for user_id: {:?}", user_id);
        secure_vaults.filter(user_id.eq(user_id))
            .load::<SecureVault>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_secure_vault(pool: &DbPool, secure_vault_id: Uuid, user_id: Uuid) -> Result<SecureVault, AppError> {
        use crate::schema::secure_vaults::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Getting secure vault with id: {:?} for user_id: {:?}", secure_vault_id, user_id);
        secure_vaults.filter(id.eq(secure_vault_id).and(user_id.eq(user_id)))
            .first::<SecureVault>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_secure_vault(pool: &DbPool, secure_vault_id: Uuid, update_data: UpdateSecureVault, user_id: Uuid) -> Result<SecureVault, AppError> {
        use crate::schema::secure_vaults::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Updating secure vault with id: {:?} for user_id: {:?}", secure_vault_id, user_id);
        diesel::update(secure_vaults.filter(id.eq(secure_vault_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_secure_vault(pool: &DbPool, secure_vault_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::secure_vaults::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Deleting secure vault with id: {:?} for user_id: {:?}", secure_vault_id, user_id);
        diesel::delete(secure_vaults.filter(id.eq(secure_vault_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map(|_| ())
            .map_err(AppError::DatabaseError)
    }
}