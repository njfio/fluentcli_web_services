use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{llm_stream_chat, LLMChatMessage};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct StreamChatQuery {
    pub conversation_id: Uuid,
    pub provider_id: Uuid,
}

pub async fn stream_chat(
    pool: web::Data<DbPool>,
    query: web::Query<StreamChatQuery>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let provider = ChatService::get_llm_provider(&pool, query.provider_id)?;
    let user_config = ChatService::get_user_llm_config(&pool, user.0, query.provider_id)?;

    let messages = ChatService::get_messages(&pool, query.conversation_id)?;

    let llm_messages: Vec<LLMChatMessage> = messages
        .into_iter()
        .map(|m| LLMChatMessage {
            role: m.role,
            content: m.content,
        })
        .collect();

    let stream = llm_stream_chat(&pool, &provider, &user_config, llm_messages).await;

    // Collect the entire response
    let full_response = stream
        .try_fold(String::new(), |mut acc, chunk| async move {
            acc.push_str(&chunk);
            Ok(acc)
        })
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

    // Save the full response to the database
    ChatService::create_message(
        &pool,
        query.conversation_id,
        "assistant".to_string(),
        Value::String(full_response.clone()),
    )?;

    // Create a new stream from the full response
    let response_stream =
        futures::stream::once(async move { Ok::<_, AppError>(web::Bytes::from(full_response)) });

    Ok(HttpResponse::Ok().streaming::<_, AppError>(response_stream))
}
