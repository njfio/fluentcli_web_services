use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    title: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    conversation_id: Uuid,
    role: String,
    content: String,
}

#[derive(Deserialize)]
pub struct CreateAttachmentRequest {
    message_id: Uuid,
    file_type: String,
    file_path: String,
}

#[derive(Deserialize)]
pub struct CreateLLMProviderRequest {
    name: String,
    api_endpoint: String,
}

#[derive(Deserialize)]
pub struct CreateUserLLMConfigRequest {
    provider_id: Uuid,
    api_key: String,
}

pub async fn create_conversation(
    pool: web::Data<DbPool>,
    req: web::Json<CreateConversationRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!(
        "Creating conversation for user: {:?}, title: {}",
        *user_id, req.title
    );
    let conversation =
        web::block(move || ChatService::create_conversation(&pool, *user_id, req.title.clone()))
            .await
            .map_err(|e| {
                error!("Error creating conversation: {:?}", e);
                AppError::GenericError(Box::new(e))
            })??;

    info!("Conversation created successfully: {:?}", conversation);
    Ok(HttpResponse::Created().json(conversation))
}

pub async fn get_conversation(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conversation = web::block(move || {
        let conversation = ChatService::get_conversation(&pool, *conversation_id)?;
        if conversation.user_id != *user_id {
            return Err(AppError::Unauthorized);
        }
        Ok(conversation)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(conversation))
}

pub async fn list_conversations(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!("Listing conversations for user: {:?}", *user_id);
    let conversations = web::block(move || ChatService::list_conversations(&pool, *user_id))
        .await
        .map_err(|e| {
            error!("Error listing conversations: {:?}", e);
            AppError::GenericError(Box::new(e))
        })??;

    info!("Conversations listed successfully");
    Ok(HttpResponse::Ok().json(conversations))
}

pub async fn create_message(
    pool: web::Data<DbPool>,
    req: web::Json<CreateMessageRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!(
        "Creating message for user: {:?}, conversation: {:?}",
        *user_id, req.conversation_id
    );
    let message = web::block(move || {
        // Check if the user owns the conversation
        let conversation = ChatService::get_conversation(&pool, req.conversation_id)?;
        if conversation.user_id != *user_id {
            error!(
                "Unauthorized: User {:?} does not own conversation {:?}",
                *user_id, req.conversation_id
            );
            return Err(AppError::Unauthorized);
        }

        ChatService::create_message(
            &pool,
            req.conversation_id,
            req.role.clone(),
            req.content.clone(),
        )
    })
    .await
    .map_err(|e| {
        error!("Error creating message: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    info!("Message created successfully: {:?}", message);
    Ok(HttpResponse::Created().json(message))
}

pub async fn get_messages(
    pool: web::Data<DbPool>,
    conversation_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conversation_uuid = *conversation_id;
    let messages = web::block(move || {
        // Check if the user owns the conversation
        let conversation = ChatService::get_conversation(&pool, conversation_uuid)?;
        if conversation.user_id != *user_id {
            return Err(AppError::Unauthorized);
        }

        ChatService::get_messages(&pool, conversation_uuid)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(messages))
}

pub async fn create_attachment(
    pool: web::Data<DbPool>,
    req: web::Json<CreateAttachmentRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!(
        "Creating attachment for user: {:?}, message: {:?}",
        *user_id, req.message_id
    );
    let attachment = web::block(move || {
        // Check if the user owns the message's conversation
        let message = ChatService::get_message(&pool, req.message_id)?;
        let conversation = ChatService::get_conversation(&pool, message.conversation_id)?;
        if conversation.user_id != *user_id {
            error!(
                "Unauthorized: User {:?} does not own conversation for message {:?}",
                *user_id, req.message_id
            );
            return Err(AppError::Unauthorized);
        }

        ChatService::create_attachment(
            &pool,
            req.message_id,
            req.file_type.clone(),
            req.file_path.clone(),
        )
    })
    .await
    .map_err(|e| {
        error!("Error creating attachment: {:?}", e);
        AppError::GenericError(Box::new(e))
    })??;

    info!("Attachment created successfully: {:?}", attachment);
    Ok(HttpResponse::Created().json(attachment))
}

pub async fn get_attachments(
    pool: web::Data<DbPool>,
    message_id: web::Path<Uuid>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let message_uuid = *message_id;
    let attachments = web::block(move || {
        // Check if the user owns the message's conversation
        let message = ChatService::get_message(&pool, message_uuid)?;
        let conversation = ChatService::get_conversation(&pool, message.conversation_id)?;
        if conversation.user_id != *user_id {
            return Err(AppError::Unauthorized);
        }

        ChatService::get_attachments(&pool, message_uuid)
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(attachments))
}

pub async fn create_llm_provider(
    pool: web::Data<DbPool>,
    req: web::Json<CreateLLMProviderRequest>,
    _user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || {
        ChatService::create_llm_provider(&pool, req.name.clone(), req.api_endpoint.clone())
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(provider))
}

pub async fn get_llm_provider(
    pool: web::Data<DbPool>,
    provider_id: web::Path<Uuid>,
    _user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let provider = web::block(move || ChatService::get_llm_provider(&pool, *provider_id))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(provider))
}

pub async fn create_user_llm_config(
    pool: web::Data<DbPool>,
    req: web::Json<CreateUserLLMConfigRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    let config = web::block(move || {
        ChatService::create_user_llm_config(&pool, *user_id, req.provider_id, req.api_key.clone())
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Created().json(config))
}

pub async fn get_user_llm_config(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, provider_id) = path.into_inner();
    let config = web::block(move || ChatService::get_user_llm_config(&pool, user_id, provider_id))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(config))
}
