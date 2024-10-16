use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct AnthropicProvider;

impl LLMProviderTrait for AnthropicProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().ok_or_else(|| {
            LLMServiceError(AppError::BadRequest(
                "Model not specified for Anthropic provider".to_string(),
            ))
        })?;

        let messages: Vec<Value> = messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "role": msg.role,
                    "content": msg.content
                })
            })
            .collect();

        let request_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": 300,
            "stream": true
        });

        debug!("Anthropic request body: {:?}", request_body);

        Ok(client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Anthropic raw response: {}", response_text);
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
                                    if let Some(content) = json["delta"]["text"].as_str() {
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
