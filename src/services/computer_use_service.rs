use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::llm_provider::LLMProviderService;
use crate::services::llm_service::{LLMChatMessage, LLMService, LLMServiceError};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

pub struct ComputerUseService;

impl ComputerUseService {
    pub async fn stream_chat(
        pool: Arc<DbPool>,
        messages: Vec<LLMChatMessage>,
        user_id: Uuid,
    ) -> Pin<Box<dyn Stream<Item = Result<String, AppError>> + Send>> {
        // Get all LLM providers
        let providers = match LLMProviderService::get_llm_providers(&pool) {
            Ok(p) => p,
            Err(e) => return Box::pin(futures::stream::once(async move { Err(e) })),
        };

        // Find the claude-computer provider
        let provider = match providers
            .iter()
            .find(|p| p.provider_type == "claude-computer")
        {
            Some(p) => p.clone(),
            None => {
                return Box::pin(futures::stream::once(async {
                    Err(AppError::NotFoundError(
                        "Claude computer provider not found".to_string(),
                    ))
                }))
            }
        };

        // Get all user's LLM configs
        let configs = match LLMProviderService::list_user_llm_configs(&pool, user_id) {
            Ok(c) => c,
            Err(e) => return Box::pin(futures::stream::once(async move { Err(e) })),
        };

        // Find the config for the claude-computer provider
        let user_config = match configs.into_iter().find(|c| c.provider_id == provider.id) {
            Some(c) => c,
            None => {
                return Box::pin(futures::stream::once(async {
                    Err(AppError::NotFoundError(
                        "User config for Claude computer provider not found".to_string(),
                    ))
                }))
            }
        };

        // Use the LLMService to handle the chat
        let stream =
            LLMService::llm_stream_chat(pool, Arc::new(provider), Arc::new(user_config), messages)
                .await;

        // Convert LLMServiceError to AppError
        Box::pin(stream.map(|result| match result {
            Ok(chunk) => Ok(chunk),
            Err(LLMServiceError(e)) => Err(e),
        }))
    }
}
