use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateLLMProviderRequest {
    name: String,
    provider_type: String,
    api_endpoint: String,
    supported_modalities: Value,
    configuration: Value,
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || {
        ChatService::create_llm_provider(
            &pool,
            req.name.clone(),
            req.provider_type.clone(),
            req.api_endpoint.clone(),
            req.supported_modalities.clone(),
            req.configuration.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(provider))
}

pub async fn get_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let provider =
        web::block(move || ChatService::get_llm_provider(&pool, provider_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(provider))
}
