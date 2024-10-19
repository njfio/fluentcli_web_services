use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::LLMProvider;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::api_key_service::ApiKeyService;
use crate::services::llm_providers::{
    anthropic::AnthropicProvider, cohere::CohereProvider, dalle::DalleProvider,
    gemini::GeminiProvider, openai::OpenAIProvider, perplexity::PerplexityProvider,
};
use futures::stream::{Stream, StreamExt};
use log::{debug, error, warn};
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
    debug!(
        "Starting chat function with provider: {:?}",
        provider.provider_type
    );

    let provider_trait: Box<dyn LLMProviderTrait + Send + Sync> =
        match provider.provider_type.as_str() {
            "gpt" => Box::new(OpenAIProvider),
            "claude" => Box::new(AnthropicProvider),
            "command" => Box::new(CohereProvider),
            "gemini" => Box::new(GeminiProvider),
            _ => {
                error!("Unsupported LLM provider: {}", provider.provider_type);
                return Err(LLMServiceError(AppError::UnsupportedProviderError(
                    "Unsupported LLM provider".to_string(),
                )));
            }
        };

    debug!("Retrieving API key for id: {:?}", user_config.api_key_id);
    let api_key = ApiKeyService::get_api_key_by_id(pool, user_config.api_key_id)
        .map_err(|e| {
            error!("Failed to retrieve API key: {:?}", e);
            LLMServiceError(e)
        })?
        .ok_or_else(|| {
            error!("API key not found");
            LLMServiceError(AppError::NotFoundError("API key not found".to_string()))
        })?;

    debug!(
        "API key retrieved successfully. Key length: {}",
        api_key.key_value.len()
    );

    let request =
        provider_trait.prepare_request(&messages, &provider.configuration, &api_key.key_value)?;

    debug!("Sending request to LLM provider");
    let response = request.send().await.map_err(|e| {
        error!("Failed to send request: {:?}", e);
        LLMServiceError(AppError::ExternalServiceError(format!(
            "Failed to send request: {}",
            e
        )))
    })?;

    if !response.status().is_success() {
        let error_message = response.text().await.map_err(|e| {
            error!("Failed to get error message: {:?}", e);
            LLMServiceError(AppError::ExternalServiceError(format!(
                "Failed to get error message: {}",
                e
            )))
        })?;
        error!("LLM provider returned an error: {}", error_message);
        return Err(LLMServiceError(AppError::ExternalServiceError(format!(
            "LLM provider returned an error: {}",
            error_message
        ))));
    }

    debug!("Streaming response from LLM provider");
    let mut stream = provider_trait.stream_response(response);
    let mut full_response = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(text) => full_response.push_str(&text),
            Err(e) => {
                error!("Error while streaming response: {:?}", e);
                return Err(e);
            }
        }
    }

    debug!("Chat function completed successfully");
    Ok(full_response)
}

pub async fn llm_stream_chat(
    pool: &DbPool,
    provider: &LLMProvider,
    user_config: &UserLLMConfig,
    messages: Vec<LLMChatMessage>,
) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
    debug!(
        "Starting llm_stream_chat function with provider: {:?}",
        provider.provider_type
    );

    let provider_trait: Box<dyn LLMProviderTrait + Send + Sync> =
        match provider.provider_type.as_str() {
            "gpt" => Box::new(OpenAIProvider),
            "claude" => Box::new(AnthropicProvider),
            "command" => Box::new(CohereProvider),
            "dalle" => Box::new(DalleProvider),
            "perplexity" => Box::new(PerplexityProvider),
            "gemini" => Box::new(GeminiProvider),
            _ => {
                error!("Unsupported LLM provider: {}", provider.provider_type);
                return Box::pin(futures::stream::once(async {
                    Err(LLMServiceError(AppError::UnsupportedProviderError(
                        "Unsupported LLM provider".to_string(),
                    )))
                }));
            }
        };

    debug!("Retrieving API key for id: {:?}", user_config.api_key_id);
    let api_key = match ApiKeyService::get_api_key_by_id(pool, user_config.api_key_id) {
        Ok(Some(key)) => {
            debug!(
                "API key retrieved successfully. Key length: {}",
                key.key_value.len()
            );
            key.key_value
        }
        Ok(None) => {
            error!("API key not found");
            return Box::pin(futures::stream::once(async {
                Err(LLMServiceError(AppError::NotFoundError(
                    "API key not found".to_string(),
                )))
            }));
        }
        Err(e) => {
            error!("Failed to retrieve API key: {:?}", e);
            return Box::pin(futures::stream::once(
                async move { Err(LLMServiceError(e)) },
            ));
        }
    };

    debug!("Preparing request for LLM provider");
    let request = match provider_trait.prepare_request(&messages, &provider.configuration, &api_key)
    {
        Ok(req) => req,
        Err(e) => {
            error!("Failed to prepare request: {:?}", e);
            return Box::pin(futures::stream::once(async move { Err(e) }));
        }
    };

    debug!("Sending request to LLM provider");
    Box::pin(
        futures::stream::once(async move { request.send().await })
            .map(move |result| match result {
                Ok(response) => {
                    if !response.status().is_success() {
                        let status = response.status();
                        let error_future = async move {
                            let error_body = response
                                .text()
                                .await
                                .unwrap_or_else(|e| format!("Failed to get error body: {}", e));
                            error!(
                                "LLM API error: Status {} {}, Body: {}",
                                status.as_u16(),
                                status.as_str(),
                                error_body
                            );
                            Err(LLMServiceError(AppError::ExternalServiceError(format!(
                                "LLM API error: Status {} {}, Body: {}",
                                status.as_u16(),
                                status.as_str(),
                                error_body
                            ))))
                        };
                        Box::pin(futures::stream::once(error_future))
                            as Pin<
                                Box<
                                    dyn Stream<Item = Result<String, LLMServiceError>>
                                        + Send
                                        + 'static,
                                >,
                            >
                    } else {
                        debug!("Streaming response from LLM provider");
                        provider_trait.stream_response(response)
                    }
                }
                Err(e) => {
                    error!("Request error: {:?}", e);
                    Box::pin(futures::stream::once(async move {
                        Err(LLMServiceError(AppError::ExternalServiceError(format!(
                            "Request error: {}",
                            e
                        ))))
                    }))
                        as Pin<
                            Box<
                                dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static,
                            >,
                        >
                }
            })
            .flatten(),
    )
}
