use crate::error::AppError;
use crate::services::chat_service;
use actix_web::web::Bytes;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    message: String,
}

pub async fn send_message(
    message: web::Json<ChatMessage>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    info!("Received send_message request");
    let extensions = req.extensions();
    let user_id = match extensions.get::<Uuid>() {
        Some(id) => id,
        None => {
            warn!("User ID not found in request extensions");
            return Err(AppError::Unauthorized);
        }
    };

    info!(
        "Received chat message from user {}: {}",
        user_id, message.content
    );
    match chat_service::generate_response(&message.content).await {
        Ok(response) => {
            info!("Generated response for user {}", user_id);
            Ok(HttpResponse::Ok().json(ChatResponse { message: response }))
        }
        Err(e) => {
            error!("Error generating response: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn chat_stream(message: web::Query<ChatMessage>, req: HttpRequest) -> impl Responder {
    info!("Received chat_stream request");
    let extensions = req.extensions();
    let user_id = match extensions.get::<Uuid>() {
        Some(id) => id.to_string(),
        None => {
            warn!("User ID not found in request extensions");
            return HttpResponse::Unauthorized().finish();
        }
    };

    info!(
        "Received stream request from user {}: {}",
        user_id, message.content
    );
    let stream = chat_service::generate_stream_response(message.content.clone());

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream.map(
            move |result: Result<String, AppError>| -> Result<Bytes, actix_web::Error> {
                match result {
                    Ok(message) => Ok(Bytes::from(format!(
                        "data: {}\n\n",
                        serde_json::to_string(&ChatResponse { message }).unwrap()
                    ))),
                    Err(e) => {
                        error!("Error in stream for user {}: {:?}", user_id, e);
                        Ok(Bytes::from(format!(
                            "data: {}\n\n",
                            serde_json::to_string(&ChatResponse {
                                message: "Error occurred".to_string()
                            })
                            .unwrap()
                        )))
                    }
                }
            },
        ))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .wrap(crate::utils::auth::Auth)
            .route("", web::post().to(send_message))
            .route("/stream", web::get().to(chat_stream)),
    );
}
