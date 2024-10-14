use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{LLMChatMessage, LLMService};
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateLLMProviderRequest {
    name: String,
    provider_type: String,
    api_endpoint: String,
    supported_modalities: Value,
    configuration: Value,
}

#[derive(Deserialize, Debug)]
pub struct LLMChatRequest {
    conversation_id: Uuid,
    provider_id: Uuid,
    messages: Vec<LLMChatMessage>,
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || {
        LLMService::create_llm_provider(
            &pool,
            req.name.clone(),
            req.provider_type.clone(),
            req.api_endpoint.clone(),
            req.supported_modalities.clone(),
            req.configuration.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(provider))
}

pub async fn llm_chat(
    pool: web::Data<DbPool>,
    req: web::Json<LLMChatRequest>,
) -> Result<HttpResponse, AppError> {
    info!("Starting llm_chat function");
    info!("Request: {:?}", req);

    let response = LLMService::llm_chat(&pool, req.provider_id, req.messages.clone()).await?;
    info!("Received LLM response: {:?}", response);

    // Parse the LLM response
    let parsed_response: Value = serde_json::from_str(&response).map_err(|e| {
        error!("Failed to parse LLM response: {:?}", e);
        AppError::SerializationError(format!("Failed to parse LLM response: {}", e))
    })?;

    info!("Parsed LLM response: {:?}", parsed_response);

    // Extract the content from the parsed response
    let content = parsed_response.as_str().unwrap_or(&response);

    info!("Extracted content: {:?}", content);

    // Create a proper JSON value for the content
    let content_json = serde_json::json!(content);

    // Save the LLM response as a new message
    let new_message = web::block(move || {
        info!("Attempting to create new message with LLM response");
        info!("Conversation ID: {:?}", req.conversation_id);
        info!("Content JSON: {:?}", content_json);
        ChatService::create_message(
            &pool,
            req.conversation_id,
            "assistant".to_string(),
            content_json,
        )
    })
    .await
    .map_err(|e| {
        error!("Error in web::block for create_message: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    info!("New message created: {:?}", new_message);

    Ok(HttpResponse::Ok().json(new_message))
}

pub async fn get_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let provider =
        web::block(move || LLMService::get_llm_provider(&pool, provider_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(provider))
}

pub async fn get_llm_providers(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let providers = web::block(move || LLMService::get_llm_providers(&pool))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(providers))
}
