use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream};
use log::{debug, error};
use reqwest::{multipart, Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct StabilityError {
    id: String,
    name: String,
    errors: Vec<String>,
}

pub struct StabilityProvider;

impl StabilityProvider {
    fn handle_error_response(status: u16, body: &str) -> LLMServiceError {
        let error_msg = match serde_json::from_str::<StabilityError>(body) {
            Ok(error) => error.errors.join(", "),
            Err(_) => body.to_string(),
        };

        let error = match status {
            400 => format!("Invalid parameters: {}", error_msg),
            401 => "Invalid API key".to_string(),
            403 => format!("Content moderation flagged request: {}", error_msg),
            404 => format!("Resource not found: {}", error_msg),
            422 => format!("Request rejected: {}", error_msg),
            429 => "Rate limit exceeded: maximum 150 requests per 10 seconds".to_string(),
            500 => format!("Internal server error: {}", error_msg),
            _ => format!("Unexpected error ({}): {}", status, error_msg),
        };

        LLMServiceError(AppError::ExternalServiceError(error))
    }

    async fn save_image_to_temp(image_data: &[u8]) -> Result<String, LLMServiceError> {
        // Create a temporary directory if it doesn't exist
        let temp_dir = std::env::var("TEMP_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp/stability_images"));

        fs::create_dir_all(&temp_dir).await.map_err(|e| {
            error!("Failed to create temp directory: {:?}", e);
            LLMServiceError(AppError::InternalServerError)
        })?;

        // Generate a unique filename
        let file_name = format!("{}.png", Uuid::new_v4());
        let file_path = temp_dir.join(&file_name);

        // Save the image
        fs::write(&file_path, image_data).await.map_err(|e| {
            error!("Failed to write image file: {:?}", e);
            LLMServiceError(AppError::InternalServerError)
        })?;

        // Return a URL that points to our backend
        let api_url =
            std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        Ok(format!("{}/temp-images/{}", api_url, file_name))
    }
}

impl LLMProviderTrait for StabilityProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<RequestBuilder, LLMServiceError> {
        let client = Client::new();

        // Get the last user message as the prompt
        let prompt = messages
            .last()
            .map(|msg| msg.content.trim())
            .filter(|content| !content.is_empty())
            .ok_or_else(|| {
                LLMServiceError(AppError::BadRequest(
                    "No valid prompt found in the most recent message for StabilityAI".to_string(),
                ))
            })?;

        // Create multipart form with required prompt
        let mut form = multipart::Form::new().text("prompt", prompt.to_string());

        // Add optional parameters if provided in config
        if let Some(output_format) = config.get("output_format").and_then(|v| v.as_str()) {
            form = form.text("output_format", output_format.to_string());
        }

        debug!("StabilityAI request parameters: prompt={}", prompt);

        Ok(client
            .post("https://api.stability.ai/v2beta/stable-image/generate/ultra")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Accept", "image/*") // Request raw image bytes
            .multipart(form))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        // Create a blocking runtime for the async operation
        let rt = tokio::runtime::Runtime::new().unwrap();
        let temp_url = rt.block_on(Self::save_image_to_temp(response_text.as_bytes()))?;
        Ok(format!("IMAGE_URL:{}", temp_url))
    }

    fn stream_response(
        &self,
        response: Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
        Box::pin(stream::once(async move {
            if !response.status().is_success() {
                let status = response.status().as_u16();
                let text = response.text().await.unwrap_or_default();
                return Err(Self::handle_error_response(status, &text));
            }

            let bytes = response.bytes().await.map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to read StabilityAI response: {}",
                    e
                )))
            })?;

            let temp_url = Self::save_image_to_temp(&bytes).await?;
            Ok(format!("IMAGE_URL:{}", temp_url))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prepare_request() {
        let provider = StabilityProvider;
        let config = json!({
            "output_format": "png"
        });

        let messages = vec![LLMChatMessage {
            role: "user".to_string(),
            content: "Generate an image of a cat".to_string(),
        }];

        let result = provider.prepare_request(&messages, &config, "test_key");
        assert!(result.is_ok());
    }
}
