use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, warn};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct OpenAIProvider;

impl LLMProviderTrait for OpenAIProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().unwrap_or("gpt-3.5-turbo");

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true
        });

        debug!("OpenAI request body: {:?}", request_body);
        debug!("Using API key: {}", api_key);

        Ok(client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Parsing OpenAI response: {}", response_text);

        // First, try to parse as a JSON object
        if let Ok(response_json) = serde_json::from_str::<Value>(response_text) {
            debug!("Parsed OpenAI response JSON: {:?}", response_json);

            // Check for the expected structure
            if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
                return Ok(content.to_string());
            }

            // If the expected structure is not found, look for any string content
            if let Some(content) = response_json.as_str() {
                return Ok(content.to_string());
            }

            // If no string content is found, return the entire JSON as a string
            return Ok(response_json.to_string());
        }

        // If it's not valid JSON, return the raw text
        warn!("OpenAI response is not valid JSON, returning raw text");
        Ok(response_text.to_string())
    }

    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>> {
        Box::pin(
            stream::unfold(response, |mut response| async move {
                match response.chunk().await {
                    Ok(Some(chunk)) => {
                        let text = String::from_utf8_lossy(&chunk);
                        debug!("Received chunk: {}", text);
                        let lines = text.split('\n');
                        let mut result = String::new();
                        for line in lines {
                            if line.starts_with("data: ") {
                                let json_str = line.trim_start_matches("data: ");
                                if json_str == "[DONE]" {
                                    return Some((Ok(result), response));
                                }
                                if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                                    if let Some(content) =
                                        json["choices"][0]["delta"]["content"].as_str()
                                    {
                                        result.push_str(content);
                                    }
                                }
                            }
                        }
                        if !result.is_empty() {
                            Some((Ok(result), response))
                        } else {
                            Some((Ok(String::new()), response))
                        }
                    }
                    Ok(None) => None,
                    Err(e) => {
                        error!("Error in stream_response: {:?}", e);
                        Some((
                            Err(LLMServiceError(AppError::ExternalServiceError(
                                e.to_string(),
                            ))),
                            response,
                        ))
                    }
                }
            })
            .filter(|result| futures::future::ready(!matches!(result, Ok(ref s) if s.is_empty()))),
        )
    }
}
