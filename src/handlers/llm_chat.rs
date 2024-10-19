use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{chat as llm_chat, LLMChatMessage, LLMServiceError};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct LLMChatRequest {
    pub user_llm_config_id: Uuid,
    pub conversation_id: Uuid,
    pub messages: Vec<LLMChatMessage>,
}

#[derive(Serialize)]
pub struct LLMChatResponse {
    status: String,
    response: String,
}

pub async fn llm_chat_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LLMChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let req = req.into_inner();

    // Get the user LLM config
    let user_config = ChatService::get_user_llm_config_by_id(&pool, req.user_llm_config_id)?;

    // Get the LLM provider
    let provider = ChatService::get_llm_provider(&pool, user_config.provider_id)?;

    // Call the LLM service
    let response = llm_chat(&pool, &provider, &user_config, req.messages.clone())
        .await
        .map_err(|e: LLMServiceError| AppError::ExternalServiceError(e.to_string()))?;

    // Save the LLM response to the database
    ChatService::create_message(
        &pool,
        req.conversation_id,
        "assistant".to_string(),
        Value::String(response.clone()),
        provider.name.clone(),
        None,                   // attachment_id
        Some(response.clone()), // raw_output
        None,                   // usage_stats
    )?;

    Ok(HttpResponse::Ok().json(LLMChatResponse {
        status: "success".to_string(),
        response,
    }))
}
