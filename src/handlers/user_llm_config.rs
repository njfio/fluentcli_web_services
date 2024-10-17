use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_provider::LLMProviderService;
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserLLMConfigRequest {
    user_id: Uuid,
    provider_id: Uuid,
    api_key_id: Uuid,
}

#[derive(Deserialize)]
pub struct GetUserLLMConfigQuery {
    user_id: Uuid,
    provider_id: Uuid,
}

pub async fn create_user_llm_config(
    pool: web::Data<DbPool>,
    req: web::Json<CreateUserLLMConfigRequest>,
) -> Result<HttpResponse, AppError> {
    info!("Attempting to create user LLM config: {:?}", req);
    let config = web::block(move || {
        ChatService::create_user_llm_config(&pool, req.user_id, req.provider_id, req.api_key_id)
    })
    .await
    .map_err(|e| {
        error!("Error creating user LLM config: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    info!("User LLM config created successfully: {:?}", config);
    Ok(HttpResponse::Created().json(config))
}

pub async fn get_user_llm_config(
    pool: web::Data<DbPool>,
    query: web::Query<GetUserLLMConfigQuery>,
) -> Result<HttpResponse, AppError> {
    let config = web::block(move || {
        ChatService::get_user_llm_config(&pool, query.user_id, query.provider_id)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(config))
}

pub async fn delete_user_llm_config(
    pool: web::Data<DbPool>,
    config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!(
        "Attempting to delete user LLM config with ID: {}",
        config_id
    );
    let deleted_rows = web::block(move || {
        LLMProviderService::delete_user_llm_config(&pool, config_id.into_inner())
    })
    .await
    .map_err(|e| {
        error!("Error deleting user LLM config: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    if deleted_rows > 0 {
        info!("User LLM config deleted successfully");
        Ok(HttpResponse::NoContent().finish())
    } else {
        info!("User LLM config not found");
        Ok(HttpResponse::NotFound().finish())
    }
}
