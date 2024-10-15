use crate::error::AppError;
use crate::models::llm_provider::LLMProvider;
use crate::models::user_llm_config::UserLLMConfig;
use crate::services::llm_providers::{
    anthropic::AnthropicProvider, cohere::CohereProvider, openai::OpenAIProvider,
};
use futures::stream::{Stream, StreamExt};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::pin::Pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatMessage {
    pub role: String,
    pub content: String,
}

pub trait LLMProviderTrait {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<RequestBuilder, AppError>;
    fn parse_response(&self, response_text: &str) -> Result<String, AppError>;
    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, AppError>> + Send>>;
}

pub async fn chat(
    provider: &LLMProvider,
    user_config: &UserLLMConfig,
    messages: Vec<LLMChatMessage>,
) -> Result<String, AppError> {
    let provider_trait: Box<dyn LLMProviderTrait> = match provider.provider_type.as_str() {
        "gpt" => Box::new(OpenAIProvider),
        "claude" => Box::new(AnthropicProvider),
        "command" => Box::new(CohereProvider),
        _ => return Err(AppError::BadRequest("Unsupported LLM provider".to_string())),
    };

    let request =
        provider_trait.prepare_request(&messages, &provider.configuration, &user_config.api_key)?;

    let response = request
        .send()
        .await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

    if !response.status().is_success() {
        let error_message = response
            .text()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
        return Err(AppError::ExternalServiceError(format!(
            "LLM provider returned an error: {}",
            error_message
        )));
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
    provider: &LLMProvider,
    user_config: &UserLLMConfig,
    messages: Vec<LLMChatMessage>,
) -> impl Stream<Item = Result<String, AppError>> {
    let provider_trait: Box<dyn LLMProviderTrait> = match provider.provider_type.as_str() {
        "gpt" => Box::new(OpenAIProvider),
        "claude" => Box::new(AnthropicProvider),
        "command" => Box::new(CohereProvider),
        _ => {
            return futures::stream::once(async {
                Err(AppError::ExternalServiceError(
                    "Unsupported LLM provider".to_string(),
                ))
            })
            .boxed()
        }
    };

    let request = match provider_trait.prepare_request(
        &messages,
        &provider.configuration,
        &user_config.api_key,
    ) {
        Ok(req) => req,
        Err(e) => {
            return futures::stream::once(async move { Err(e) }).boxed();
        }
    };

    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            return futures::stream::once(async move {
                Err(AppError::ExternalServiceError(format!(
                    "Request error: {}",
                    e
                )))
            })
            .boxed();
        }
    };

    if !response.status().is_success() {
        let error = AppError::ExternalServiceError(format!(
            "LLM API error: Status {} {}",
            response.status().as_u16(),
            response.status().as_str()
        ));
        return futures::stream::once(async move { Err(error) }).boxed();
    }

    provider_trait.stream_response(response).boxed()
}
