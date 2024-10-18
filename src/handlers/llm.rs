use crate::db::db::DbPool;
use crate::error::AppError;
use crate::handlers::llm_chat::{llm_chat_handler, LLMChatRequest};
use crate::handlers::stream_chat::{stream_chat, StreamChatQuery};
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};

pub async fn llm_chat(
    pool: web::Data<DbPool>,
    req: web::Json<LLMChatRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    llm_chat_handler(pool, req, user).await
}

pub async fn llm_stream_chat(
    pool: web::Data<DbPool>,
    query: web::Query<StreamChatQuery>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    stream_chat(pool, query, user).await
}

// Re-export functions from llm_provider
pub use crate::handlers::llm_provider::{create_llm_provider, get_llm_provider};

// Implement get_llm_providers function
pub async fn get_llm_providers(pool: web::Data<DbPool>) -> Result<HttpResponse, AppError> {
    let providers =
        web::block(move || crate::services::chat_service::ChatService::get_llm_providers(&pool))
            .await
            .map_err(|e| AppError::GenericError(Box::new(e)))??;

    Ok(HttpResponse::Ok().json(providers))
}
