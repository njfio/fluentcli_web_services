use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_service::LLMChatMessage;
use crate::services::ComputerUseService;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use futures::{Stream, StreamExt};
use serde::Deserialize;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
pub struct ComputerUseChatRequest {
    pub messages: Vec<LLMChatMessage>,
}

pub async fn computer_use_chat(
    pool: web::Data<DbPool>,
    req: web::Json<ComputerUseChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let (tx, mut rx) = mpsc::channel(100);
    let pool_arc = Arc::new(pool.get_ref().clone());

    // Spawn stream handling task
    actix_web::rt::spawn({
        let pool_arc = Arc::clone(&pool_arc);
        let messages = req.messages.clone();
        let user_id = user.0;
        async move {
            let mut stream = ComputerUseService::stream_chat(pool_arc, messages, user_id).await;
            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
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
    });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(futures::stream::poll_fn(move |cx| rx.poll_recv(cx))))
}
