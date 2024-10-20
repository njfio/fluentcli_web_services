use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct DalleProvider;

impl LLMProviderTrait for DalleProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().ok_or_else(|| {
            LLMServiceError(AppError::BadRequest(
                "Model not specified for DALL-E provider".to_string(),
            ))
        })?;
        let size = config["size"].as_str().unwrap_or("1024x1024");
        let quality = config["quality"].as_str().unwrap_or("standard");

        // For DALL-E, we'll use the last non-empty message as the prompt
        let prompt = messages
            .iter()
            .filter(|msg| !msg.content.trim().is_empty())
            .last()
            .map(|msg| msg.content.trim())
            .ok_or_else(|| {
                LLMServiceError(AppError::BadRequest(
                    "No valid prompt found for DALL-E".to_string(),
                ))
            })?;

        let request_body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "n": 1,
            "size": size,
            "quality": quality
        });

        debug!("DALL-E request body: {:?}", request_body);

        Ok(client
            .post("https://api.openai.com/v1/images/generations")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("DALL-E raw response: {}", response_text);
        let response: Value = serde_json::from_str(response_text).map_err(|e| {
            LLMServiceError(AppError::ExternalServiceError(format!(
                "Failed to parse DALL-E response: {}",
                e
            )))
        })?;

        response["data"][0]["url"]
            .as_str()
            .map(|url| url.to_string())
            .ok_or_else(|| {
                LLMServiceError(AppError::ExternalServiceError(
                    "No image URL found in DALL-E response".to_string(),
                ))
            })
    }

    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
        Box::pin(stream::once(async move {
            let body = response.text().await.map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to read DALL-E response: {}",
                    e
                )))
            })?;
            DalleProvider.parse_response(&body)
        }))
    }
}