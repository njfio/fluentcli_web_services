use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{llm_stream_chat, LLMChatMessage, LLMServiceError};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
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

    let full_response = Arc::new(Mutex::new(String::new()));
    let full_response_clone = Arc::clone(&full_response);

    let (tx, rx) = oneshot::channel();

    let response_stream = stream.then(move |chunk_result| {
        let full_response = Arc::clone(&full_response);
        async move {
            match chunk_result {
                Ok(chunk) => {
                    let mut full = full_response.lock().unwrap();
                    full.push_str(&chunk);
                    Ok(web::Bytes::from(chunk))
                }
                Err(e) => Err(AppError::ExternalServiceError(e.to_string())),
            }
        }
    });

    // Use a separate task to save the full response to the database
    let pool_clone = pool.clone();
    let conversation_id = query.conversation_id;
    actix_web::rt::spawn(async move {
        let full_response = full_response_clone.lock().unwrap().clone();
        if !full_response.trim().is_empty() {
            match ChatService::create_message(
                &pool_clone,
                conversation_id,
                "assistant".to_string(),
                Value::String(full_response),
            ) {
                Ok(_) => {
                    println!("Message saved to database successfully");
                    let _ = tx.send(());
                }
                Err(e) => {
                    eprintln!("Error saving message to database: {:?}", e);
                    let _ = tx.send(());
                }
            }
        } else {
            let _ = tx.send(());
        }
    });

    // Wait for the database write to complete before sending the response
    let response = HttpResponse::Ok().streaming(response_stream);
    rx.await.expect("Failed to receive completion signal");

    Ok(response)
}
