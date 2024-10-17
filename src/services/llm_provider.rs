use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::schema::{llm_providers, user_llm_configs};
use diesel::prelude::*;
use uuid::Uuid;

pub struct LLMProviderService;
impl LLMProviderService {
    pub fn create_llm_provider(
        pool: &DbPool,
        new_provider: NewLLMProvider,
    ) -> Result<LLMProvider, AppError> {
        let conn = &mut pool.get().unwrap();
        diesel::insert_into(llm_providers::table)
            .values(&new_provider)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<LLMProvider, AppError> {
        let conn = &mut pool.get().unwrap();
        llm_providers::table
            .find(provider_id)
            .first(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_llm_provider(
        pool: &DbPool,
        provider_id: Uuid,
        updated_provider: NewLLMProvider,
    ) -> Result<LLMProvider, AppError> {
        let conn = &mut pool.get().unwrap();
        diesel::update(llm_providers::table.find(provider_id))
            .set(&updated_provider)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }
    pub fn delete_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<usize, AppError> {
        let conn = &mut pool.get().unwrap();
        conn.transaction(|conn| {
            // First, delete associated user_llm_configs
            diesel::delete(
                user_llm_configs::table.filter(user_llm_configs::provider_id.eq(provider_id)),
            )
            .execute(conn)
            .map_err(AppError::DatabaseError)?;

            // Then, delete the LLM provider
            diesel::delete(llm_providers::table.find(provider_id))
                .execute(conn)
                .map_err(AppError::DatabaseError)
        })
    }

    pub fn create_user_llm_config(
        pool: &DbPool,
        new_config: NewUserLLMConfig,
    ) -> Result<UserLLMConfig, AppError> {
        let conn = &mut pool.get().unwrap();
        diesel::insert_into(user_llm_configs::table)
            .values(&new_config)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_user_llm_config(
        pool: &DbPool,
        config_id: Uuid,
    ) -> Result<Option<UserLLMConfig>, AppError> {
        let conn = &mut pool.get().unwrap();
        user_llm_configs::table
            .find(config_id)
            .first(conn)
            .optional()
            .map_err(AppError::DatabaseError)
    }

    pub fn update_user_llm_config(
        pool: &DbPool,
        config_id: Uuid,
        updated_config: NewUserLLMConfig,
    ) -> Result<UserLLMConfig, AppError> {
        let conn = &mut pool.get().unwrap();
        diesel::update(user_llm_configs::table.find(config_id))
            .set(&updated_config)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_user_llm_config(pool: &DbPool, config_id: Uuid) -> Result<usize, AppError> {
        let conn = &mut pool.get().unwrap();
        diesel::delete(user_llm_configs::table.find(config_id))
            .execute(conn)
            .map_err(AppError::DatabaseError)
    }
}
