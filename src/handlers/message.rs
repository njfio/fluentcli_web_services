use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    conversation_id: Uuid,
    role: String,
    content: String, // Changed from Value to String
    provider_model: String,
    raw_output: Option<String>,
    usage_stats: Option<Value>,
}

pub async fn create_message(
    pool: web::Data<DbPool>,
    req: web::Json<CreateMessageRequest>,
) -> Result<HttpResponse, AppError> {
    let message = ChatService::create_message_with_attachments(
        &pool,
        req.conversation_id,
        req.role.clone(),
        req.content.clone(),
        req.provider_model.clone(),
        req.raw_output.clone(),
        req.usage_stats.clone(),
    )
    .await?;

    Ok(HttpResponse::Created().json(message))
}

pub async fn get_messages(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let messages =
        web::block(move || ChatService::get_messages(&pool, conversation_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(messages))
}

pub async fn get_message(
    pool: web::Data<DbPool>,
    message_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message = web::block(move || ChatService::get_message(&pool, message_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(message))
}
