use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::services::llm_provider::LLMProviderService;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateLLMProviderRequest {
    pub user_id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: serde_json::Value,
    pub configuration: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserLLMConfigRequest {
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub api_key_id: Uuid,
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<impl Responder, AppError> {
    let new_provider = NewLLMProvider {
        user_id: req.user_id,
        name: req.name.clone(),
        provider_type: req.provider_type.clone(),
        api_endpoint: req.api_endpoint.clone(),
        supported_modalities: req.supported_modalities.clone(),
        configuration: req.configuration.clone(),
    };

    let provider = web::block(move || LLMProviderService::create_llm_provider(&pool, new_provider))
        .await
        .map_err(|e| AppError::InternalServerError)?
        .map_err(|e| e)?;

    Ok(HttpResponse::Created().json(provider))
}

pub async fn get_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let provider =
        web::block(move || LLMProviderService::get_llm_provider(&pool, provider_id.into_inner()))
            .await
            .map_err(|e| AppError::InternalServerError)?
            .map_err(|e| e)?;

    Ok(HttpResponse::Ok().json(provider))
}

pub async fn update_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<impl Responder, AppError> {
    let updated_provider = NewLLMProvider {
        user_id: req.user_id,
        name: req.name.clone(),
        provider_type: req.provider_type.clone(),
        api_endpoint: req.api_endpoint.clone(),
        supported_modalities: req.supported_modalities.clone(),
        configuration: req.configuration.clone(),
    };

    let provider = web::block(move || {
        LLMProviderService::update_llm_provider(&pool, provider_id.into_inner(), updated_provider)
    })
    .await
    .map_err(|e| AppError::InternalServerError)?
    .map_err(|e| e)?;

    Ok(HttpResponse::Ok().json(provider))
}

pub async fn delete_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let deleted = web::block(move || {
        LLMProviderService::delete_llm_provider(&pool, provider_id.into_inner())
    })
    .await
    .map_err(|e| AppError::InternalServerError)?
    .map_err(|e| e)?;

    if deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub async fn create_user_llm_config(
    pool: web::Data<DbPool>,
    req: web::Json<CreateUserLLMConfigRequest>,
) -> Result<impl Responder, AppError> {
    let new_config = NewUserLLMConfig {
        user_id: req.user_id,
        provider_id: req.provider_id,
        api_key_id: req.api_key_id,
    };

    let config = web::block(move || LLMProviderService::create_user_llm_config(&pool, new_config))
        .await
        .map_err(|e| AppError::InternalServerError)?;

    match config {
        Ok(config) => Ok(HttpResponse::Created().json(config)),
        Err(AppError::ProviderNotFound(provider_id)) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Provider with ID {} not found", provider_id)
            })))
        }
        Err(e) => Err(e),
    }
}

pub async fn get_user_llm_config(
    pool: web::Data<DbPool>,
    config_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let result =
        web::block(move || LLMProviderService::get_user_llm_config(&pool, config_id.into_inner()))
            .await
            .map_err(|e| AppError::InternalServerError)?
            .map_err(|e| e)?;

    match result {
        Some(config) => Ok(HttpResponse::Ok().json(config)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn update_user_llm_config(
    pool: web::Data<DbPool>,
    config_id: web::Path<Uuid>,
    req: web::Json<CreateUserLLMConfigRequest>,
) -> Result<impl Responder, AppError> {
    let updated_config = NewUserLLMConfig {
        user_id: req.user_id,
        provider_id: req.provider_id,
        api_key_id: req.api_key_id,
    };

    let config = web::block(move || {
        LLMProviderService::update_user_llm_config(&pool, config_id.into_inner(), updated_config)
    })
    .await
    .map_err(|e| AppError::InternalServerError)?
    .map_err(|e| e)?;

    Ok(HttpResponse::Ok().json(config))
}

pub async fn delete_user_llm_config(
    pool: web::Data<DbPool>,
    config_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let deleted = web::block(move || {
        LLMProviderService::delete_user_llm_config(&pool, config_id.into_inner())
    })
    .await
    .map_err(|e| AppError::InternalServerError)?
    .map_err(|e| e)?;

    if deleted > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
