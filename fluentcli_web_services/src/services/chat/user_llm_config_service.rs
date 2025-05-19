use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::schema::user_llm_configs;
use diesel::prelude::*;
use log::{error, info};
use uuid::Uuid;

pub struct UserLLMConfigService;

impl UserLLMConfigService {
    pub fn create_user_llm_config(
        pool: &DbPool,
        _user_id: Uuid,
        _provider_id: Uuid,
        _api_key_id: Uuid,
        _description: Option<String>,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!(
            "Creating new user LLM config - user_id: {:?}, provider_id: {:?}, api_key_id: {:?}, description: {:?}",
            _user_id, _provider_id, _api_key_id, _description
        );

        let new_config = NewUserLLMConfig {
            user_id: _user_id,
            provider_id: _provider_id,
            api_key_id: _api_key_id,
            description: _description,
        };

        let result = diesel::insert_into(user_llm_configs)
            .values(&new_config)
            .get_result::<UserLLMConfig>(&mut pool.get()?);

        match result {
            Ok(config) => {
                info!("Successfully created user LLM config: {:?}", config);
                Ok(config)
            }
            Err(e) => {
                error!("Error creating user LLM config: {:?}", e);
                error!("Attempted to insert: {:?}", new_config);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn get_user_llm_config(
        pool: &DbPool,
        _user_id: Uuid,
        _provider_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!(
            "Fetching user LLM config - user_id: {:?}, provider_id: {:?}",
            _user_id, _provider_id
        );

        user_llm_configs
            .filter(user_id.eq(_user_id))
            .filter(provider_id.eq(_provider_id))
            .first::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching user LLM config: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_user_llm_config_by_id(
        pool: &DbPool,
        _config_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!("Fetching user LLM config by id: {:?}", _config_id);

        user_llm_configs
            .find(_config_id)
            .first::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching user LLM config by id: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn list_user_llm_configs(
        pool: &DbPool,
        _user_id: Uuid,
    ) -> Result<Vec<UserLLMConfig>, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!("Listing user LLM configs for user_id: {:?}", _user_id);

        user_llm_configs
            .filter(user_id.eq(_user_id))
            .load::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error listing user LLM configs: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn update_user_llm_config(
        pool: &DbPool,
        _config_id: Uuid,
        updated_config: NewUserLLMConfig,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!("Updating user LLM config with id: {:?}", _config_id);

        diesel::update(user_llm_configs.find(_config_id))
            .set(&updated_config)
            .get_result::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error updating user LLM config: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn delete_user_llm_config(pool: &DbPool, _config_id: Uuid) -> Result<usize, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!("Deleting user LLM config with id: {:?}", _config_id);

        diesel::delete(user_llm_configs.find(_config_id))
            .execute(&mut pool.get()?)
            .map_err(|e| {
                error!("Error deleting user LLM config: {:?}", e);
                AppError::DatabaseError(e)
            })
    }
}
