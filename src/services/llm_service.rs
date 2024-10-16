use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::LLMProvider;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::api_key_service::ApiKeyService;
use crate::services::llm_providers::{
    anthropic::AnthropicProvider, cohere::CohereProvider, openai::OpenAIProvider,
};
use futures::stream::{Stream, StreamExt};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::pin::Pin;

#[derive(Debug)]
pub struct LLMServiceError(pub AppError);

impl fmt::Display for LLMServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LLMServiceError {}

impl From<AppError> for LLMServiceError {
    fn from(error: AppError) -> Self {
        LLMServiceError(error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatMessage {
    pub role: String,
    pub content: String,
}

pub trait LLMProviderTrait: Send + Sync {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<RequestBuilder, LLMServiceError>;
    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError>;
    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>>;
}

pub async fn chat(
    pool: &DbPool,
    provider: &LLMProvider,
    user_config: &UserLLMConfig,
    messages: Vec<LLMChatMessage>,
) -> Result<String, LLMServiceError> {
    let provider_trait: Box<dyn LLMProviderTrait + Send + Sync> =
        match provider.provider_type.as_str() {
            "gpt" => Box::new(OpenAIProvider),
            "claude" => Box::new(AnthropicProvider),
            "command" => Box::new(CohereProvider),
            _ => {
                return Err(LLMServiceError(AppError::UnsupportedProviderError(
                    "Unsupported LLM provider".to_string(),
                )))
            }
        };

    let api_key = ApiKeyService::get_api_key_by_id(pool, user_config.api_key_id)
        .map_err(LLMServiceError)?
        .ok_or_else(|| LLMServiceError(AppError::NotFoundError("API key not found".to_string())))?;

    let request =
        provider_trait.prepare_request(&messages, &provider.configuration, &api_key.key_value)?;

    let response = request.send().await.map_err(|e| {
        LLMServiceError(AppError::ExternalServiceError(format!(
            "Failed to send request: {}",
            e
        )))
    })?;

    if !response.status().is_success() {
        let error_message = response.text().await.map_err(|e| {
            LLMServiceError(AppError::ExternalServiceError(format!(
                "Failed to get error message: {}",
                e
            )))
        })?;
        return Err(LLMServiceError(AppError::ExternalServiceError(format!(
            "LLM provider returned an error: {}",
            error_message
        ))));
    }

    let mut stream = provider_trait.stream_response(response);
    let mut full_response = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(text) => full_response.push_str(&text),
            Err(e) => return Err(e),
        }
    }

    Ok(full_response)
}

pub async fn llm_stream_chat(
    pool: &DbPool,
    provider: &LLMProvider,
    user_config: &UserLLMConfig,
    messages: Vec<LLMChatMessage>,
) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
    let provider_trait: Box<dyn LLMProviderTrait + Send + Sync> =
        match provider.provider_type.as_str() {
            "gpt" => Box::new(OpenAIProvider),
            "claude" => Box::new(AnthropicProvider),
            "command" => Box::new(CohereProvider),
            _ => {
                return Box::pin(futures::stream::once(async {
                    Err(LLMServiceError(AppError::UnsupportedProviderError(
                        "Unsupported LLM provider".to_string(),
                    )))
                }))
            }
        };

    let api_key = match ApiKeyService::get_api_key_by_id(pool, user_config.api_key_id) {
        Ok(Some(key)) => key.key_value,
        Ok(None) => {
            return Box::pin(futures::stream::once(async {
                Err(LLMServiceError(AppError::NotFoundError(
                    "API key not found".to_string(),
                )))
            }))
        }
        Err(e) => {
            return Box::pin(futures::stream::once(
                async move { Err(LLMServiceError(e)) },
            ))
        }
    };

    let request = match provider_trait.prepare_request(&messages, &provider.configuration, &api_key)
    {
        Ok(req) => req,
        Err(e) => return Box::pin(futures::stream::once(async move { Err(e) })),
    };

    Box::pin(
        futures::stream::once(async move { request.send().await })
            .map(move |result| match result {
                Ok(response) => {
                    if !response.status().is_success() {
                        let error = LLMServiceError(AppError::ExternalServiceError(format!(
                            "LLM API error: Status {} {}",
                            response.status().as_u16(),
                            response.status().as_str()
                        )));
                        Box::pin(futures::stream::once(async move { Err(error) }))
                            as Pin<
                                Box<
                                    dyn Stream<Item = Result<String, LLMServiceError>>
                                        + Send
                                        + 'static,
                                >,
                            >
                    } else {
                        provider_trait.stream_response(response)
                    }
                }
                Err(e) => Box::pin(futures::stream::once(async move {
                    Err(LLMServiceError(AppError::ExternalServiceError(format!(
                        "Request error: {}",
                        e
                    ))))
                }))
                    as Pin<
                        Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>,
                    >,
            })
            .flatten(),
    )
}
