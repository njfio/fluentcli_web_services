use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{chat as llm_chat, LLMChatMessage};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct LLMChatRequest {
    pub provider_id: Uuid,
    pub conversation_id: Uuid,
    pub messages: Vec<LLMChatMessage>,
}

#[derive(Serialize)]
pub struct LLMChatResponse {
    response: String,
}

pub async fn llm_chat_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LLMChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let req = req.into_inner();
    let provider = web::block({
        let pool = pool.clone();
        let provider_id = req.provider_id;
        move || ChatService::get_llm_provider(&pool, provider_id)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    let user_config = web::block({
        let pool = pool.clone();
        let provider_id = req.provider_id;
        let user_id = user.0;
        move || ChatService::get_user_llm_config(&pool, user_id, provider_id)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    let response = llm_chat(&provider, &user_config, req.messages.clone())
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

    // Save the LLM response to the database
    let new_message = web::block({
        let pool = pool.clone();
        let conversation_id = req.conversation_id;
        let response_clone = response.clone();
        move || {
            ChatService::create_message(
                &pool,
                conversation_id,
                "assistant".to_string(),
                Value::String(response_clone),
            )
        }
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(LLMChatResponse { response }))
}

pub async fn llm_stream_chat_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LLMChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Implement streaming chat logic here
    unimplemented!("Streaming chat not implemented yet")
}
