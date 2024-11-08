use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_service::LLMChatMessage;
use crate::services::ComputerUseService;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::{Stream, StreamExt};
use log::{debug, error, info};
use serde::Deserialize;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
pub struct ComputerUseChatRequest {
    pub messages: Vec<LLMChatMessage>,
}

pub async fn computer_use_chat(
    pool: web::Data<DbPool>,
    req: web::Json<ComputerUseChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let (tx, mut rx) = mpsc::channel(100);
    let pool_arc = Arc::new(pool.get_ref().clone());

    // Spawn stream handling task
    actix_web::rt::spawn({
        let pool_arc = Arc::clone(&pool_arc);
        let messages = req.messages.clone();
        let user_id = user.0;
        async move {
            let mut stream = ComputerUseService::stream_chat(pool_arc, messages, user_id).await;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        if tx.send(Ok(web::Bytes::from(chunk))).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx
                            .send(Err(actix_web::error::ErrorInternalServerError(
                                e.to_string(),
                            )))
                            .await;
                        break;
                    }
                }
            }
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(futures::stream::poll_fn(move |cx| rx.poll_recv(cx))))
}

pub async fn serve_screenshot(
    req: HttpRequest,
    filename: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("Serving screenshot request for: {}", filename);
    info!("Request headers: {:?}", req.headers());

    // Get the ATTACHMENT_DIR from environment or use default
    let attachment_dir =
        std::env::var("ATTACHMENT_DIR").unwrap_or_else(|_| "/app/attachments".to_string());
    info!("Using attachment directory: {}", attachment_dir);

    let screenshots_dir = PathBuf::from(&attachment_dir).join("screenshots");
    info!("Screenshots directory: {:?}", screenshots_dir);

    // Verify screenshots directory exists
    if !screenshots_dir.exists() {
        error!(
            "Screenshots directory does not exist: {:?}",
            screenshots_dir
        );
        // Try to create the directory
        if let Err(e) = std::fs::create_dir_all(&screenshots_dir) {
            error!("Failed to create screenshots directory: {}", e);
            return Err(AppError::ConfigError(format!(
                "Failed to create screenshots directory: {}",
                e
            )));
        }
        info!("Created screenshots directory");
    }

    // List directory contents for debugging
    match std::fs::read_dir(&screenshots_dir) {
        Ok(entries) => {
            info!("Contents of screenshots directory:");
            for entry in entries {
                match entry {
                    Ok(entry) => info!("  {:?}", entry.path()),
                    Err(e) => error!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => error!("Failed to read screenshots directory: {}", e),
    }

    // Construct and validate the file path
    let filepath = screenshots_dir.join(filename.as_ref());
    info!("Looking for screenshot at: {:?}", filepath);

    // Security check: ensure the file path is within the screenshots directory
    if !filepath.starts_with(&screenshots_dir) {
        error!("Invalid screenshot path: {:?}", filepath);
        return Err(AppError::BadRequestError(
            "Invalid screenshot path".to_string(),
        ));
    }

    // Check if file exists
    if !filepath.exists() {
        error!("Screenshot file not found: {:?}", filepath);
        return Err(AppError::NotFoundError(format!(
            "Screenshot not found: {}",
            filename
        )));
    }

    // Get file metadata
    match std::fs::metadata(&filepath) {
        Ok(metadata) => {
            info!(
                "File metadata - size: {} bytes, readonly: {}, modified: {:?}",
                metadata.len(),
                metadata.permissions().readonly(),
                metadata.modified().ok()
            );
        }
        Err(e) => error!("Failed to get file metadata: {}", e),
    }

    // Read and serve the file
    match std::fs::read(&filepath) {
        Ok(content) => {
            info!(
                "Successfully read screenshot, size: {} bytes",
                content.len()
            );
            Ok(HttpResponse::Ok()
                .insert_header(header::ContentType::png())
                .insert_header(header::CacheControl(vec![
                    header::CacheDirective::NoCache,
                    header::CacheDirective::NoStore,
                ]))
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .insert_header(("Access-Control-Allow-Headers", "Authorization"))
                .insert_header(("Access-Control-Allow-Methods", "GET, OPTIONS"))
                .body(content))
        }
        Err(e) => {
            error!("Failed to read screenshot at {:?}: {}", filepath, e);
            Err(AppError::ConfigError(format!(
                "Failed to read screenshot: {}",
                e
            )))
        }
    }
}
