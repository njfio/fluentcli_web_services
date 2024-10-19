use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct PerplexityProvider;

impl LLMProviderTrait for PerplexityProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().ok_or_else(|| {
            LLMServiceError(AppError::BadRequest(
                "Model not specified for Perplexity provider".to_string(),
            ))
        })?;

        // Format messages for Perplexity API, ensuring alternating user and assistant roles
        let mut formatted_messages: Vec<Value> = Vec::new();
        let mut last_role = String::new();

        for msg in messages.iter().rev() {
            let role = match msg.role.as_str() {
                "user" => "user",
                "assistant" => "assistant",
                "system" => "system",
                _ => "user", // Default to user for unknown roles
            };

            if role != last_role || formatted_messages.is_empty() {
                formatted_messages.push(serde_json::json!({
                    "role": role,
                    "content": msg.content
                }));
                last_role = role.to_string();
            }

            if formatted_messages.len() >= 3 {
                break; // Limit to last 3 messages (1 user, 1 assistant, 1 user)
            }
        }

        formatted_messages.reverse(); // Reverse to maintain chronological order

        let request_body = serde_json::json!({
            "model": model,
            "messages": formatted_messages,
            "max_tokens": config["max_tokens"].as_u64().unwrap_or(1024),
            "temperature": config["temperature"].as_f64().unwrap_or(0.7),
            "top_p": config["top_p"].as_f64().unwrap_or(0.9),
            "stream": true,
        });

        debug!("Perplexity request body: {:?}", request_body);

        Ok(client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Perplexity raw response: {}", response_text);
        let response: Value = serde_json::from_str(response_text).map_err(|e| {
            LLMServiceError(AppError::ExternalServiceError(format!(
                "Failed to parse Perplexity response: {}",
                e
            )))
        })?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                LLMServiceError(AppError::ExternalServiceError(
                    "No content found in Perplexity response".to_string(),
                ))
            })?;

        Ok(content.to_string())
    }

    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send + 'static>> {
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
