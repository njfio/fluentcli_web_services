use crate::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::LLMProvider;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::chat_service::ChatService;
use crate::services::llm_service::{LLMChatMessage, LLMService, LLMServiceError};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use futures::{Stream, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct StreamChatRequest {
    pub user_llm_config_id: Uuid,
    pub provider_id: Uuid,
    pub conversation_id: Uuid,
    pub messages: Vec<LLMChatMessage>,
}

pub async fn stream_chat(
    pool: web::Data<DbPool>,
    req: web::Json<StreamChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let provider = Arc::new(ChatService::get_llm_provider(&pool, req.provider_id)?);
    let user_config = Arc::new(ChatService::get_user_llm_config(
        &pool,
        user.0,
        req.provider_id,
    )?);

    let (tx, rx) = mpsc::channel(100);
    let full_response = Arc::new(Mutex::new(String::new()));
    let full_response_clone = Arc::clone(&full_response);

    let pool_arc = Arc::new(pool.get_ref().clone());

    actix_web::rt::spawn({
        let pool_arc = Arc::clone(&pool_arc);
        let provider = Arc::clone(&provider);
        let user_config = Arc::clone(&user_config);
        let messages = req.messages.clone();
        async move {
            if let Err(e) = handle_llm_stream(
                pool_arc,
                provider,
                user_config,
                messages,
                tx,
                full_response_clone,
            )
            .await
            {
                eprintln!("Error in handle_llm_stream: {:?}", e);
            }
        }
    });

    let response_stream = create_response_stream(rx);

    // Use a separate task to save the full response to the database
    let conversation_id = req.conversation_id;
    let provider_model = provider.name.clone();

    actix_web::rt::spawn({
        let pool_arc = Arc::clone(&pool_arc);
        let full_response = Arc::clone(&full_response);
        async move {
            let full_response = full_response.lock().await.clone();
            if !full_response.trim().is_empty() {
                match ChatService::create_message(
                    &pool_arc,
                    conversation_id,
                    "assistant".to_string(),
                    Value::String(full_response.clone()),
                    provider_model,
                    None,                        // attachment_id
                    Some(full_response.clone()), // raw_output
                    None,                        // usage_stats
                ) {
                    Ok(_) => println!("Message saved to database successfully"),
                    Err(e) => eprintln!("Error saving message to database: {:?}", e),
                }
            }
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(response_stream))
}

async fn handle_llm_stream(
    pool: Arc<DbPool>,
    provider: Arc<LLMProvider>,
    user_config: Arc<UserLLMConfig>,
    messages: Vec<LLMChatMessage>,
    tx: mpsc::Sender<Result<web::Bytes, actix_web::Error>>,
    full_response: Arc<Mutex<String>>,
) -> Result<(), AppError> {
    let stream = LLMService::llm_stream_chat(pool, provider, user_config, messages).await;
    process_stream(stream, tx, full_response).await;
    Ok(())
}

async fn process_stream(
    mut stream: Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>>,
    tx: mpsc::Sender<Result<web::Bytes, actix_web::Error>>,
    full_response: Arc<Mutex<String>>,
) {
    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                let mut full = full_response.lock().await;
                full.push_str(&chunk);
                if tx.send(Ok(web::Bytes::from(chunk))).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                let _ = tx
                    .send(Err(actix_web::error::ErrorInternalServerError(
                        e.to_string(),
                    )))
                    .await;
                break;
            }
        }
    }
}

fn create_response_stream(
    mut rx: mpsc::Receiver<Result<web::Bytes, actix_web::Error>>,
) -> impl Stream<Item = Result<web::Bytes, actix_web::Error>> {
    futures::stream::poll_fn(move |cx| rx.poll_recv(cx))
}

// Add this implementation to convert LLMServiceError to AppError
impl From<LLMServiceError> for AppError {
    fn from(error: LLMServiceError) -> Self {
        AppError::ExternalServiceError(error.to_string())
    }
}
