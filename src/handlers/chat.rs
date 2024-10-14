use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{LLMChatMessage, LLMService};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    conversation_id: Uuid,
    role: String,
    content: Value,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    title: String,
}

#[derive(Deserialize)]
pub struct CreateLLMProviderRequest {
    name: String,
    provider_type: String,
    api_endpoint: String,
    supported_modalities: Value,
    configuration: Value,
}

#[derive(Deserialize)]
pub struct CreateAttachmentRequest {
    message_id: Uuid,
    file_type: String,
    file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserLLMConfigRequest {
    user_id: Uuid,
    provider_id: Uuid,
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LLMChatRequest {
    conversation_id: Uuid,
    provider_id: Uuid,
    messages: Vec<LLMChatMessage>,
}

#[derive(Deserialize)]
pub struct GetUserLLMConfigQuery {
    user_id: Uuid,
    provider_id: Uuid,
}
pub async fn create_conversation(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    req: web::Json<CreateConversationRequest>,
) -> Result<HttpResponse, AppError> {
    let conversation =
        web::block(move || ChatService::create_conversation(&pool, user.0, req.title.clone()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(conversation))
}

pub async fn create_message(
    pool: web::Data<DbPool>,
    req: web::Json<CreateMessageRequest>,
) -> Result<HttpResponse, AppError> {
    let message = web::block(move || {
        ChatService::create_message(
            &pool,
            req.conversation_id,
            req.role.clone(),
            req.content.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(message))
}

pub async fn get_conversation(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conversation =
        web::block(move || ChatService::get_conversation(&pool, conversation_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(conversation))
}

pub async fn list_conversations(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let conversations = web::block(move || ChatService::list_conversations(&pool, user.0))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(conversations))
}

pub async fn get_messages(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let messages =
        web::block(move || ChatService::get_messages(&pool, conversation_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(messages))
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || {
        ChatService::create_llm_provider(
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

pub async fn get_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let provider =
        web::block(move || ChatService::get_llm_provider(&pool, provider_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(provider))
}

pub async fn delete_conversation(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    web::block(move || ChatService::delete_conversation(&pool, conversation_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_message(
    pool: web::Data<DbPool>,
    message_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message = web::block(move || ChatService::get_message(&pool, message_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(message))
}

pub async fn create_attachment(
    pool: web::Data<DbPool>,
    req: web::Json<CreateAttachmentRequest>,
) -> Result<HttpResponse, AppError> {
    let attachment = web::block(move || {
        ChatService::create_attachment(
            &pool,
            req.message_id,
            req.file_type.clone(),
            req.file_path.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(attachment))
}

pub async fn get_attachments(
    pool: web::Data<DbPool>,
    message_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let attachments =
        web::block(move || ChatService::get_attachments(&pool, message_id.into_inner()))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(attachments))
}

pub async fn create_user_llm_config(
    pool: web::Data<DbPool>,
    req: web::Json<CreateUserLLMConfigRequest>,
) -> Result<HttpResponse, AppError> {
    info!("Attempting to create user LLM config: {:?}", req);
    let config = web::block(move || {
        ChatService::create_user_llm_config(
            &pool,
            req.user_id,
            req.provider_id,
            req.api_key.clone(),
        )
    })
    .await
    .map_err(|e| {
        error!("Error creating user LLM config: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    info!("User LLM config created successfully: {:?}", config);
    Ok(HttpResponse::Created().json(config))
}

pub async fn get_user_llm_config(
    pool: web::Data<DbPool>,
    query: web::Query<GetUserLLMConfigQuery>,
) -> Result<HttpResponse, AppError> {
    let config = web::block(move || {
        ChatService::get_user_llm_config(&pool, query.user_id, query.provider_id)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(config))
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
    let content = parsed_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            error!("Failed to extract content from LLM response");
            AppError::SerializationError("Failed to extract content from LLM response".to_string())
        })?;

    info!("Extracted content: {:?}", content);

    // Create a proper JSON value for the content
    let content_json = serde_json::json!(content);

    // Clone the values we need for the closure
    let conversation_id = req.conversation_id;
    let pool_clone = pool.clone();

    // Save the LLM response as a new message
    let new_message = web::block(move || {
        info!("Attempting to create new message with LLM response");
        info!("Conversation ID: {:?}", conversation_id);
        info!("Content JSON: {:?}", content_json);
        ChatService::create_message(
            &pool_clone,
            conversation_id,
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

    // Fetch the updated messages to confirm the new message was added
    let updated_messages = web::block(move || ChatService::get_messages(&pool, conversation_id))
        .await
        .map_err(|e| {
            error!("Error in web::block for get_messages: {:?}", e);
            AppError::GenericError(Box::new(e))
        })??;

    info!("Updated messages: {:?}", updated_messages);

    Ok(HttpResponse::Ok().json(new_message))
}

pub async fn get_llm_providers(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let providers = web::block(move || LLMService::get_llm_providers(&pool))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(providers))
}
