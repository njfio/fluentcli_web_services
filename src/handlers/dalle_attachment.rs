use crate::db::DbPool;
use crate::error::AppError;
use crate::services::attachment_service::AttachmentService;
use actix_web::{web, HttpResponse, Result};
use log::{debug, error, info};
use uuid::Uuid;

pub async fn get_dalle_attachment(
    pool: web::Data<DbPool>,
    attachment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!("Fetching DALL-E attachment with ID: {}", attachment_id);
    let attachment =
        web::block(move || AttachmentService::get_attachment(&pool, attachment_id.into_inner()))
            .await
            .map_err(|e| {
                error!("Failed to get DALL-E attachment from database: {:?}", e);
                AppError::GenericError(Box::new(e))
            })??;

    debug!(
        "DALL-E attachment retrieved from database: {:?}",
        attachment
    );
    info!("DALL-E attachment file path: {}", attachment.file_path);

    if attachment
        .file_path
        .starts_with("https://oaidalleapiprodscus.blob.core.windows.net")
    {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(serde_json::json!({ "url": attachment.file_path, "type": "dalle" })))
    } else {
        Err(AppError::BadRequest("Not a DALL-E attachment".into()))
    }
}
