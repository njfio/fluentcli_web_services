use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    title: String,
    #[serde(default = "default_mode")]
    mode: String,
}

fn default_mode() -> String {
    "chat".to_string()
}

pub async fn create_conversation(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    req: web::Json<CreateConversationRequest>,
) -> Result<HttpResponse, AppError> {
    let conversation = web::block(move || {
        ChatService::create_conversation(&pool, user.0, req.title.clone(), req.mode.clone())
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(conversation))
}

pub async fn get_conversation(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conversation =
        web::block(move || ChatService::get_conversation(&pool, conversation_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(conversation))
}

pub async fn list_conversations(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let conversations = web::block(move || ChatService::list_conversations(&pool, user.0))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(conversations))
}

pub async fn delete_conversation(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    web::block(move || ChatService::delete_conversation(&pool, conversation_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::NoContent().finish())
}
