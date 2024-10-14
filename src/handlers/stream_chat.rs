use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMService};
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StreamChatRequest {
    provider_id: Uuid,
    messages: String, // This will be a JSON string that we'll parse
}

pub async fn stream_chat(
    pool: web::Data<DbPool>,
    query: web::Query<StreamChatRequest>,
) -> impl Responder {
    // Parse the messages JSON string
    let messages: Vec<LLMChatMessage> = match serde_json::from_str(&query.messages) {
        Ok(msgs) => msgs,
        Err(e) => {
            return HttpResponse::BadRequest().json(format!("Invalid messages format: {}", e))
        }
    };

    let stream = LLMService::llm_stream_chat(&pool, query.provider_id, messages).await;

    match stream {
        Ok(stream) => {
            let mapped_stream = stream.map(|chunk| chunk.map(|content| web::Bytes::from(content)));
            HttpResponse::Ok()
                .content_type("text/event-stream")
                .streaming(mapped_stream)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
