use crate::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use actix_web::{web, HttpResponse};
use log::{debug, error, info};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateMessageRequest {
    conversation_id: Uuid,
    role: String,
    content: String,
    provider_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_stats: Option<Value>,
}

pub async fn create_message(
    pool: web::Data<DbPool>,
    req: web::Json<CreateMessageRequest>,
) -> Result<HttpResponse, AppError> {
    info!("Creating message: {:?}", req);

    // Check if attachment_id is provided
    if let Some(attachment_id) = req.attachment_id {
        debug!("Message has attachment_id: {}", attachment_id);
        // Use the regular create_message method that supports attachment_id
        let message = web::block(move || {
            ChatService::create_message(
                &pool,
                req.conversation_id,
                req.role.clone(),
                serde_json::Value::String(req.content.clone()),
                req.provider_model.clone(),
                Some(attachment_id),
                req.raw_output.clone(),
                req.usage_stats.clone(),
            )
        })
        .await
        .map_err(|e| {
            error!("Error creating message with attachment: {:?}", e);
            AppError::GenericError(Box::new(e))
        })??;

        return Ok(HttpResponse::Created().json(message));
    }

    // If no attachment_id, use the create_message_with_attachments method
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

pub async fn delete_message(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (conversation_id, message_id) = path.into_inner();
    web::block(move || ChatService::delete_message(&pool, conversation_id, message_id))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::NoContent().finish())
}
