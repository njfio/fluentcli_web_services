use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::api_key::ApiKey;
use crate::services::api_key_service::ApiKeyService;

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    description: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ApiKeyResponse {
    id: Uuid,
    key_value: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(api_key: ApiKey) -> Self {
        Self {
            id: api_key.id,
            key_value: api_key.key_value,
            description: api_key.description,
            created_at: api_key.created_at,
            updated_at: api_key.updated_at,
            expires_at: api_key.expires_at,
        }
    }
}

pub async fn create_api_key(
    pool: web::Data<DbPool>,
    req: web::Json<CreateApiKeyRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let key_value = Uuid::new_v4().to_string();
    let api_key = web::block(move || {
        ApiKeyService::create_api_key(
            &pool,
            *user_id,
            key_value,
            req.description.clone(),
            req.expires_at,
        )
    })
    .await
    .map_err(|e| AppError::InternalServerError)?;

    Ok(HttpResponse::Created().json(ApiKeyResponse::from(api_key?)))
}

pub async fn list_api_keys(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let api_keys = web::block(move || ApiKeyService::list_api_keys_for_user(&pool, *user_id))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    let response: Vec<ApiKeyResponse> = api_keys.into_iter().map(ApiKeyResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_api_key(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool, id.into_inner()))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == *user_id => {
            Ok(HttpResponse::Ok().json(ApiKeyResponse::from(key)))
        }
        Some(_) => Err(AppError::Unauthorized),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_api_key(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    id: web::Path<Uuid>,
    req: web::Json<CreateApiKeyRequest>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let id_value = *id;
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool, id_value))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == *user_id => {
            let updated_key = web::block(move || {
                ApiKeyService::update_api_key(
                    &pool_clone,
                    id_value,
                    req.description.clone(),
                    req.expires_at,
                )
            })
            .await
            .map_err(|_| AppError::InternalServerError)??;

            Ok(HttpResponse::Ok().json(ApiKeyResponse::from(updated_key)))
        }
        Some(_) => Err(AppError::Unauthorized),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_api_key(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let id_value = *id;
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool, id_value))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == *user_id => {
            let deleted = web::block(move || ApiKeyService::delete_api_key(&pool_clone, id_value))
                .await
                .map_err(|_| AppError::InternalServerError)??;

            if deleted {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Err(AppError::NotFound)
            }
        }
        Some(_) => Err(AppError::Unauthorized),
        None => Err(AppError::NotFound),
    }
}
