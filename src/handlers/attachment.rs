use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::AttachmentType;
use crate::services::attachment_service::AttachmentService;
use actix_web::{web, HttpResponse, Result};
use log::{debug, error, info};
use mime_guess::from_path;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
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
    let attachment_type = match req.file_type.as_str() {
        "Text" => AttachmentType::Text,
        "Document" => AttachmentType::Document,
        "Video" => AttachmentType::Video,
        "Image" => AttachmentType::Image,
        "Audio" => AttachmentType::Audio,
        _ => return Err(AppError::BadRequest("Invalid attachment type".into())),
    };

    let attachment = web::block(move || {
        AttachmentService::create_attachment(
            &pool,
            req.message_id,
            attachment_type,
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
        web::block(move || AttachmentService::get_attachments(&pool, message_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(attachments))
}

pub async fn get_attachment(
    pool: web::Data<DbPool>,
    attachment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!("Fetching attachment with ID: {}", attachment_id);
    let attachment =
        web::block(move || AttachmentService::get_attachment(&pool, attachment_id.into_inner()))
            .await
            .map_err(|e| {
                error!("Failed to get attachment from database: {:?}", e);
                AppError::GenericError(Box::new(e))
            })??;

    debug!("Attachment retrieved from database: {:?}", attachment);
    info!("Attachment file path: {}", attachment.file_path);

    let mut file = File::open(&attachment.file_path).map_err(|e| {
        error!("Failed to open file: {:?}", e);
        AppError::InternalServerError
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| {
        error!("Failed to read file: {:?}", e);
        AppError::InternalServerError
    })?;

    let content_type = from_path(&attachment.file_path)
        .first_or_octet_stream()
        .to_string();

    info!("Serving attachment with content type: {}", content_type);
    debug!("File size: {} bytes", buffer.len());

    Ok(HttpResponse::Ok().content_type(content_type).body(buffer))
}

pub async fn delete_attachment(
    pool: web::Data<DbPool>,
    attachment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    web::block(move || AttachmentService::delete_attachment(&pool, attachment_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::NoContent().finish())
}
