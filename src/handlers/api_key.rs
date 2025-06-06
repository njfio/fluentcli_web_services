use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DbPool;
use crate::error::AppError;
use crate::models::api_key::ApiKey;
use crate::services::api_key_service::ApiKeyService;

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    key_value: String,
    description: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateApiKeyRequest {
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
    let user_id = *user_id;
    let key_value = req.key_value.clone();
    let description = req.description.clone();
    let expires_at = req.expires_at;

    let api_key = web::block(move || {
        ApiKeyService::create_api_key(&pool, user_id, key_value, description, expires_at)
    })
    .await
    .map_err(|e| AppError::InternalServerError)?;

    let response = ApiKeyResponse::from(api_key?);
    debug!("Created API key value: {}", response.key_value);
    Ok(HttpResponse::Created().json(response))
}

pub async fn list_api_keys(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    let api_keys = web::block(move || ApiKeyService::list_api_keys_for_user(&pool, user_id))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    let response: Vec<ApiKeyResponse> = api_keys.into_iter().map(ApiKeyResponse::from).collect();
    debug!(
        "Listed API keys: {:?}",
        response.iter().map(|k| &k.key_value).collect::<Vec<_>>()
    );
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_api_key(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    let id = id.into_inner();
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool, id))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == user_id => {
            let response = ApiKeyResponse::from(key);
            debug!("Retrieved API key value: {}", response.key_value);
            Ok(HttpResponse::Ok().json(response))
        }
        Some(_) => Err(AppError::Unauthorized),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_api_key(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    id: web::Path<Uuid>,
    req: web::Json<UpdateApiKeyRequest>,
) -> Result<impl Responder, AppError> {
    let user_id = *user_id;
    let id = id.into_inner();
    let description = req.description.clone();
    let expires_at = req.expires_at;

    let pool_clone1 = pool.clone();
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool_clone1, id))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == user_id => {
            let pool_clone2 = pool.clone();
            let updated_key = web::block(move || {
                ApiKeyService::update_api_key(&pool_clone2, id, description, expires_at)
            })
            .await
            .map_err(|_| AppError::InternalServerError)??;

            let response = ApiKeyResponse::from(updated_key);
            debug!("Updated API key value: {}", response.key_value);
            Ok(HttpResponse::Ok().json(response))
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
    let user_id = *user_id;
    let id = id.into_inner();

    let pool_clone1 = pool.clone();
    let api_key = web::block(move || ApiKeyService::get_api_key_by_id(&pool_clone1, id))
        .await
        .map_err(|_| AppError::InternalServerError)??;

    match api_key {
        Some(key) if key.user_id == user_id => {
            let pool_clone2 = pool.clone();
            let deleted = web::block(move || ApiKeyService::delete_api_key(&pool_clone2, id))
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
