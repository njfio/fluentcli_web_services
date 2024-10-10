use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_service::{self, LLMService};
use actix_web::web::Bytes;
use actix_web::{web, Error, HttpResponse};
use futures::stream::{Stream, StreamExt};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl From<ChatMessage> for llm_service::ChatMessage {
    fn from(msg: ChatMessage) -> Self {
        llm_service::ChatMessage {
            role: msg.role,
            content: msg.content,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StreamChatRequest {
    provider_id: Uuid,
    messages: Vec<ChatMessage>,
}

pub async fn stream_chat(
    pool: web::Data<DbPool>,
    req: web::Json<StreamChatRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!("Streaming chat response for user: {:?}", *user_id);

    // Log the request data
    debug!(
        "Request data: provider_id: {:?}, messages: {:?}",
        req.provider_id, req.messages
    );

    let llm_messages: Vec<llm_service::ChatMessage> =
        req.messages.iter().cloned().map(Into::into).collect();

    debug!("Converted messages: {:?}", llm_messages);

    let stream = match LLMService::stream_chat(&pool, req.provider_id, llm_messages).await {
        Ok(s) => {
            info!("Stream initialized successfully");
            s
        }
        Err(e) => {
            error!("Error initializing stream chat: {:?}", e);
            return Err(e);
        }
    };

    let mapped_stream: Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> =
        Box::pin(stream.map(|chunk_result| match chunk_result {
            Ok(content) => {
                debug!("Received content chunk: {}", content);
                let json_content = json!({
                    "choices": [
                        {
                            "delta": {
                                "content": content
                            }
                        }
                    ]
                });
                let event = format!("data: {}\n\n", json_content);
                Ok(Bytes::from(event))
            }
            Err(e) => {
                error!("Error in stream: {:?}", e);
                let error_json = json!({
                    "error": {
                        "message": format!("{:?}", e)
                    }
                });
                let error_event = format!("data: {}\n\n", error_json);
                Ok(Bytes::from(error_event))
            }
        }));

    info!("Stream initialized, sending response");
    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(mapped_stream))
}
