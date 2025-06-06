use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream};
use log::{debug, error, info};
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct LeonardoInitialResponse {
    sdGenerationJob: SdGenerationJob,
}

#[derive(Debug, Deserialize)]
struct SdGenerationJob {
    generationId: String,
    apiCreditCost: i32,
}

#[derive(Debug, Deserialize)]
struct GeneratedImage {
    url: String,
    nsfw: bool,
    id: String,
}

#[derive(Debug, Deserialize)]
struct GenerationResponse {
    generations_by_pk: GenerationsByPk,
}

#[derive(Debug, Deserialize)]
struct GenerationsByPk {
    generated_images: Vec<GeneratedImage>,
    status: String,
}

pub struct LeonardoProvider;

impl LeonardoProvider {
    async fn get_image_url(generation_id: &str, api_key: &str) -> Result<String, LLMServiceError> {
        let client = Client::new();
        let url = format!(
            "https://cloud.leonardo.ai/api/rest/v1/generations/{}",
            generation_id
        );

        let mut attempts = 0;
        let max_attempts = 30; // 30 attempts * 2 seconds = 60 seconds timeout

        while attempts < max_attempts {
            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("accept", "application/json")
                .send()
                .await
                .map_err(|e| {
                    LLMServiceError(AppError::ExternalServiceError(format!(
                        "Failed to fetch generation: {}",
                        e
                    )))
                })?;

            let generation: GenerationResponse = response.json().await.map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to parse generation response: {}",
                    e
                )))
            })?;

            if generation.generations_by_pk.status == "COMPLETE" {
                if let Some(image) = generation.generations_by_pk.generated_images.first() {
                    return Ok(format!("IMAGE_URL:{}", image.url));
                }
            }

            attempts += 1;
            sleep(Duration::from_secs(2)).await;
        }

        Err(LLMServiceError(AppError::ExternalServiceError(
            "Generation timed out".to_string(),
        )))
    }
}

impl LLMProviderTrait for LeonardoProvider {
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
                    "No valid prompt found in the most recent message for Leonardo".to_string(),
                ))
            })?;

        // Create a mutable copy of the config
        let mut request_body = config.clone();

        // Add or update required fields
        request_body["prompt"] = json!(prompt);

        // Set model ID based on model name or use default
        if let Some("phoenix") = request_body.get("model").and_then(|m| m.as_str()) {
            request_body["modelId"] = json!("6b645e3a-d64f-4341-a6d8-7a3690fbf042");
        } else {
            request_body["modelId"] = json!("6b645e3a-d64f-4341-a6d8-7a3690fbf042");
            // Default to Phoenix
        }
        request_body.as_object_mut().unwrap().remove("model"); // Remove the model field as we use modelId

        debug!("Leonardo request body: {:?}", request_body);

        Ok(client
            .post("https://cloud.leonardo.ai/api/rest/v1/generations")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Leonardo raw response: {}", response_text);
        let initial: LeonardoInitialResponse =
            serde_json::from_str(response_text).map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to parse initial response: {}",
                    e
                )))
            })?;

        Ok(format!(
            "GENERATION_ID:{}",
            initial.sdGenerationJob.generationId
        ))
    }

    fn stream_response(
        &self,
        response: Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
        Box::pin(stream::once(async move {
            let text = response.text().await.map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to read Leonardo response: {}",
                    e
                )))
            })?;

            let initial: LeonardoInitialResponse = serde_json::from_str(&text).map_err(|e| {
                LLMServiceError(AppError::ExternalServiceError(format!(
                    "Failed to parse initial response: {}",
                    e
                )))
            })?;

            // Get the image URL using the generation ID
            Self::get_image_url(
                &initial.sdGenerationJob.generationId,
                "8e36a900-1a36-4827-b5da-4f32f5bc99e9",
            )
            .await
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prepare_request() {
        let provider = LeonardoProvider;
        let config = json!({
            "width": 1472,
            "height": 832,
            "num_images": 4,
            "contrast": 3.5,
            "alchemy": true,
            "enhancePrompt": true,
            "model": "phoenix",
            "styleUUID": "111dc692-d470-4eec-b791-3475abac4c46",
            "ultra": true,
            "custom_option": "value"
        });

        let messages = vec![LLMChatMessage {
            role: "user".to_string(),
            content: "Generate an image of a cat".to_string(),
        }];

        let result = provider.prepare_request(&messages, &config, "test_key");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_initial_response() {
        let provider = LeonardoProvider;
        let response = r#"{
            "sdGenerationJob": {
                "generationId": "bbadc920-3395-4246-9a92-570710c5ce99",
                "apiCreditCost": 24
            }
        }"#;

        let result = provider.parse_response(response);
        assert!(result.is_ok());
        assert!(result
            .unwrap()
            .contains("bbadc920-3395-4246-9a92-570710c5ce99"));
    }
}
