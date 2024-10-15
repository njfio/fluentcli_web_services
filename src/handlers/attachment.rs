use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateAttachmentRequest {
    message_id: Uuid,
    file_type: String,
    file_path: String,
}

pub async fn create_attachment(
    pool: web::Data<DbPool>,
    req: web::Json<CreateAttachmentRequest>,
) -> Result<HttpResponse, AppError> {
    let attachment = web::block(move || {
        ChatService::create_attachment(
            &pool,
            req.message_id,
            req.file_type.clone(),
            req.file_path.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(attachment))
}

pub async fn get_attachments(
    pool: web::Data<DbPool>,
    message_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let attachments =
        web::block(move || ChatService::get_attachments(&pool, message_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(attachments))
}
