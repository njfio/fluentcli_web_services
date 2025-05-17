use crate::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::LLMProvider;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::api_key_service::ApiKeyService;
use crate::services::llm_providers;
use futures::stream::{Stream, StreamExt};
use log::{debug, error, info};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::pin::Pin;
use std::sync::Arc;

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

pub struct LLMService;

impl LLMService {
    pub async fn chat(
        pool: &DbPool,
        provider: &LLMProvider,
        user_config: &UserLLMConfig,
        messages: Vec<LLMChatMessage>,
    ) -> Result<String, LLMServiceError> {
        info!(
            "Starting chat function with provider: {:?}",
            provider.provider_type
        );

        let api_key = Self::get_api_key(pool, user_config).await?;

        let client = Client::new();
        let request = Self::prepare_request(&client, provider, &messages, &api_key)?;

        let response = request.send().await.map_err(|e| {
            error!("Failed to send request: {:?}", e);
            LLMServiceError(AppError::ExternalServiceError(e.to_string()))
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
            return Err(LLMServiceError(AppError::ExternalServiceError(
                error_message,
            )));
        }

        let response_text = response.text().await.map_err(|e| {
            error!("Failed to get response text: {:?}", e);
            LLMServiceError(AppError::ExternalServiceError(format!(
                "Failed to get response text: {}",
                e
            )))
        })?;

        Ok(response_text)
    }

    pub async fn llm_stream_chat(
        pool: Arc<DbPool>,
        provider: Arc<LLMProvider>,
        user_config: Arc<UserLLMConfig>,
        messages: Vec<LLMChatMessage>,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>> {
        info!(
            "Starting llm_stream_chat function with provider: {:?}",
            provider.provider_type
        );

        let api_key = match Self::get_api_key(&pool, &user_config).await {
            Ok(key) => key,
            Err(e) => return Box::pin(futures::stream::once(async move { Err(e) })),
        };

        let llm_provider = llm_providers::get_provider(&provider.provider_type);

        let client = Client::new();
        let request =
            match llm_provider.prepare_request(&messages, &provider.configuration, &api_key) {
                Ok(req) => req,
                Err(e) => {
                    error!("Failed to prepare request: {:?}", e);
                    return Box::pin(futures::stream::once(async move { Err(e) }));
                }
            };

        info!("Sending request to LLM provider");
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
                                    Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>,
                                >
                        } else {
                            info!("Streaming response from LLM provider");
                            llm_provider.stream_response(response)
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
                            as Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>>
                    }
                })
                .flatten(),
        )
    }

    async fn get_api_key(
        pool: &DbPool,
        user_config: &UserLLMConfig,
    ) -> Result<String, LLMServiceError> {
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

        Ok(api_key.key_value)
    }

    fn prepare_request(
        client: &Client,
        provider: &LLMProvider,
        messages: &[LLMChatMessage],
        api_key: &str,
    ) -> Result<RequestBuilder, LLMServiceError> {
        // Implement provider-specific request preparation here
        // For now, we'll return a placeholder implementation
        Ok(client
            .post(&provider.api_endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&messages))
    }

    fn stream_response(
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>> {
        Box::pin(futures::stream::unfold(
            response,
            |mut response| async move {
                match response.chunk().await {
                    Ok(Some(chunk)) => {
                        let chunk_str = String::from_utf8_lossy(&chunk).to_string();
                        Some((Ok(chunk_str), response))
                    }
                    Ok(None) => None,
                    Err(e) => Some((
                        Err(LLMServiceError(AppError::ExternalServiceError(
                            e.to_string(),
                        ))),
                        response,
                    )),
                }
            },
        ))
    }
}
