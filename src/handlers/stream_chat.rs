use crate::db::db::DbPool;
use crate::error::AppError;
use actix_web::web::Bytes;
use actix_web::{web, HttpResponse};
use futures::stream::Stream;
use log::info;
use serde::Deserialize;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct StreamChatRequest {
    content: String,
}

pub async fn stream_chat(
    pool: web::Data<DbPool>,
    req: web::Query<StreamChatRequest>,
    user_id: web::ReqData<Uuid>,
) -> Result<HttpResponse, AppError> {
    info!("Streaming chat response for user: {:?}", *user_id);

    // Here you would typically call your AI service to get a streaming response
    // For this example, we'll simulate a streaming response
    let stream = simulate_ai_response(req.content.clone());

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream))
}

fn simulate_ai_response(
    content: String,
) -> Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>> {
    let mut interval = interval(Duration::from_millis(100));
    let words: Vec<String> = content.split_whitespace().map(String::from).collect();
    let mut index = 0;

    Box::pin(async_stream::stream! {
        while index < words.len() {
            interval.tick().await;
            let response = format!("data: {}\n\n", words[index]);
            yield Ok(Bytes::from(response));
            index += 1;
        }
        yield Ok(Bytes::from("data: [DONE]\n\n"));
    })
}
