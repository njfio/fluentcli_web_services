use actix_web::http::header;
use actix_web::{delete, get, post, web, HttpResponse};
use diesel::prelude::*;
use log::{debug, error, info};
use mime_guess::from_path;
use std::fs;
use uuid::Uuid;

use crate::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::Attachment;
use crate::schema::attachments;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateAttachmentData {
    pub message_id: String,
    pub file_type: String,
    pub file_path: String,
}

#[get("/attachments/{id}")]
pub async fn get_attachment(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Fetching attachment with ID: {}", id);

    let conn = &mut pool.get().map_err(|e| AppError::R2D2Error(e))?;
    let attachment = attachments::table
        .find(id)
        .first::<Attachment>(conn)
        .map_err(|_| AppError::NotFoundError(format!("Attachment not found: {}", id)))?;

    info!("Attachment file path: {}", attachment.file_path);

    let content = fs::read(&attachment.file_path)
        .map_err(|e| AppError::TempFileError(format!("Failed to read file: {}", e)))?;

    debug!("File size: {} bytes", content.len());

    let content_type = mime_guess::from_path(&attachment.file_path)
        .first_or_octet_stream()
        .to_string();

    info!("Serving attachment with content type: {}", content_type);

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(content_type.parse().unwrap()))
        .body(content))
}

#[post("/attachments")]
pub async fn create_attachment(
    pool: web::Data<DbPool>,
    attachment_data: web::Json<CreateAttachmentData>,
) -> Result<HttpResponse, AppError> {
    let message_id = Uuid::parse_str(&attachment_data.message_id)
        .map_err(|_| AppError::BadRequestError("Invalid message ID".to_string()))?;

    let conn = &mut pool.get().map_err(|e| AppError::R2D2Error(e))?;
    let attachment = diesel::insert_into(attachments::table)
        .values((
            attachments::message_id.eq(message_id),
            attachments::file_type.eq(&attachment_data.file_type),
            attachments::file_path.eq(&attachment_data.file_path),
        ))
        .get_result::<Attachment>(conn)
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(HttpResponse::Ok().json(attachment))
}

#[get("/messages/{message_id}/attachments")]
pub async fn get_message_attachments(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    let conn = &mut pool.get().map_err(|e| AppError::R2D2Error(e))?;

    let attachments = attachments::table
        .filter(attachments::message_id.eq(message_id))
        .load::<Attachment>(conn)
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(HttpResponse::Ok().json(attachments))
}

#[delete("/attachments/{id}")]
pub async fn delete_attachment(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    info!("Deleting attachment with ID: {}", id);

    let conn = &mut pool.get().map_err(|e| AppError::R2D2Error(e))?;

    // Get the attachment first to get the file path
    let attachment = attachments::table
        .find(id)
        .first::<Attachment>(conn)
        .map_err(|_| AppError::NotFoundError(format!("Attachment not found: {}", id)))?;

    let file_path = attachment.file_path.clone();

    // Delete from database first
    diesel::delete(attachments::table.find(id))
        .execute(conn)
        .map_err(|e| AppError::DatabaseError(e))?;

    // Then try to delete the file
    // Don't fail if file is already gone, just log the error
    if let Err(e) = fs::remove_file(&file_path) {
        error!("Failed to delete attachment file {}: {:?}", file_path, e);
    } else {
        info!("Successfully deleted attachment file: {}", file_path);
    }

    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_attachment)
        .service(create_attachment)
        .service(get_message_attachments)
        .service(delete_attachment);
}
