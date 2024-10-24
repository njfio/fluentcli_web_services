use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::{Attachment, AttachmentType, NewAttachment};
use crate::schema::attachments;
use diesel::prelude::*;
use log::{debug, error, info};
use regex::Regex;
use reqwest;
use std::path::PathBuf;
use tokio::fs;
use url::Url;
use uuid::Uuid;

pub struct AttachmentService;

impl AttachmentService {
    pub fn get_attachment_dir() -> PathBuf {
        std::env::var("ATTACHMENT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/app/attachments"))
    }

    pub async fn download_and_store_image(
        pool: &DbPool,
        message_id: Uuid,
        image_url: &str,
    ) -> Result<Attachment, AppError> {
        info!("Downloading and storing image: {}", image_url);

        let image_url = if image_url.starts_with("IMAGE_URL:") {
            &image_url[10..]
        } else {
            image_url
        };
        debug!("Image URL: {}", image_url);
        // Parse the URL to ensure we're using the full URL with all query parameters
        let parsed_url = Url::parse(image_url).map_err(|e| {
            error!("Failed to parse image URL: {:?}", e);
            AppError::BadRequest(format!("Invalid image URL: {}", e))
        })?;
        debug!("Parsed URL: {}", parsed_url);
        let response = reqwest::get(parsed_url).await.map_err(|e| {
            error!("Failed to download image: {:?}", e);
            AppError::ExternalServiceError(e.to_string())
        })?;
        debug!("Response: {:?}", response);
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("image/jpeg");

        let extension = content_type.split('/').last().unwrap_or("jpg");
        debug!("Extension: {}", extension);
        let file_name = format!("{}.{}", Uuid::new_v4(), extension);
        debug!("File name: {}", file_name);
        let attachment_dir = Self::get_attachment_dir();
        debug!("Attachment directory: {:?}", attachment_dir);
        let file_path = attachment_dir.join(&file_name);
        debug!("File path: {:?}", file_path);

        fs::create_dir_all(&attachment_dir).await.map_err(|e| {
            error!("Failed to create attachments directory: {:?}", e);
            AppError::InternalServerError
        })?;

        let bytes = response.bytes().await.map_err(|e| {
            error!("Failed to read image bytes: {:?}", e);
            AppError::ExternalServiceError(e.to_string())
        })?;

        fs::write(&file_path, &bytes).await.map_err(|e| {
            error!("Failed to write image file: {:?}", e);
            AppError::InternalServerError
        })?;

        debug!(
            "Calling create_attachment with message_id: {:?}, type: {:?}, path: {:?}",
            message_id,
            AttachmentType::Image,
            file_path.to_str().unwrap()
        );

        Self::create_attachment(
            pool,
            message_id,
            AttachmentType::Image,
            file_path.to_str().unwrap().to_string(),
        )
    }

    pub fn create_attachment(
        pool: &DbPool,
        message_id: Uuid,
        attachment_type: AttachmentType,
        file_path: String,
    ) -> Result<Attachment, AppError> {
        use crate::schema::attachments::dsl;

        let new_attachment = NewAttachment {
            message_id,
            file_type: attachment_type.to_string(),
            file_path,
        };

        let conn = &mut pool.get()?;
        diesel::insert_into(dsl::attachments)
            .values(&new_attachment)
            .get_result::<Attachment>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_attachments(pool: &DbPool, msg_id: Uuid) -> Result<Vec<Attachment>, AppError> {
        use crate::schema::attachments::dsl;

        let conn = &mut pool.get()?;
        dsl::attachments
            .filter(dsl::message_id.eq(msg_id))
            .load::<Attachment>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_attachment(pool: &DbPool, attachment_id: Uuid) -> Result<Attachment, AppError> {
        use crate::schema::attachments::dsl;

        let conn = &mut pool.get()?;
        dsl::attachments
            .find(attachment_id)
            .first::<Attachment>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub async fn delete_attachment(pool: &DbPool, attachment_id: Uuid) -> Result<(), AppError> {
        use crate::schema::attachments::dsl;

        // First get the attachment to get the file path
        let attachment = Self::get_attachment(pool, attachment_id)?;
        let file_path = PathBuf::from(&attachment.file_path);

        // Delete from database
        let conn = &mut pool.get()?;
        diesel::delete(dsl::attachments.find(attachment_id))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;

        // Delete the file from disk
        if file_path.exists() {
            fs::remove_file(&file_path).await.map_err(|e| {
                error!("Failed to delete attachment file: {:?}", e);
                AppError::InternalServerError
            })?;
        }

        Ok(())
    }

    pub async fn process_and_store_images(
        pool: &DbPool,
        message_id: Uuid,
        content: &str,
    ) -> Result<Vec<Attachment>, AppError> {
        let url_regex = Regex::new(r"IMAGE_URL:(https?://\S+)").unwrap();
        let mut attachments = Vec::new();

        for cap in url_regex.captures_iter(content) {
            let url = &cap[1];
            match Self::download_and_store_image(pool, message_id, url).await {
                Ok(attachment) => attachments.push(attachment),
                Err(e) => error!("Failed to process image {}: {:?}", url, e),
            }
        }

        Ok(attachments)
    }

    pub fn replace_urls_with_attachment_ids(content: &str, attachments: &[Attachment]) -> String {
        let mut updated_content = content.to_string();
        for attachment in attachments {
            let placeholder = format!("{{{{ATTACHMENT:{}}}}}", attachment.id);
            updated_content = updated_content
                .replace(&format!("IMAGE_URL:{}", attachment.file_path), &placeholder);
        }
        updated_content
    }
}
