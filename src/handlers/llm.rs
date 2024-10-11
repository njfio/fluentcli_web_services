use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_service::{ChatMessage, LLMService};
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateLLMProviderRequest {
    name: String,
    api_endpoint: String,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    provider_id: Uuid,
    messages: Vec<ChatMessage>,
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || {
        LLMService::create_llm_provider(&pool, req.name.clone(), req.api_endpoint.clone())
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(provider))
}

pub async fn get_llm_providers(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let providers = web::block(move || LLMService::get_llm_providers(&pool))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(providers))
}

pub async fn chat(
    pool: web::Data<DbPool>,
    req: web::Json<ChatRequest>,
) -> Result<HttpResponse, AppError> {
    let response = LLMService::chat(&pool, req.provider_id, req.messages.clone()).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn stream_chat(pool: web::Data<DbPool>, req: web::Json<ChatRequest>) -> impl Responder {
    let stream = LLMService::stream_chat(&pool, req.provider_id, req.messages.clone()).await;

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
