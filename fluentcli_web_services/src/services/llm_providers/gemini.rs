use crate::error::AppError;
use crate::services::llm_providers::ProviderConfig;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, warn};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;

pub struct GeminiProvider {
    config: Value,
}

impl ProviderConfig for GeminiProvider {
    fn new(config: Value) -> Self {
        GeminiProvider { config }
    }
}

impl LLMProviderTrait for GeminiProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = self.config["model"].as_str().unwrap_or("gemini-pro");

        let formatted_messages: Vec<Value> = messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "role": match msg.role.as_str() {
                        "user" => "user",
                        "assistant" => "model",
                        "system" => "system",
                        _ => "user",
                    },
                    "parts": [{"text": msg.content}]
                })
            })
            .collect();

        let request_body = serde_json::json!({
            "contents": formatted_messages,
            "generationConfig": {
                "temperature": self.config["temperature"].as_f64().unwrap_or(0.7),
                "topK": self.config["top_k"].as_u64().unwrap_or(40),
                "topP": self.config["top_p"].as_f64().unwrap_or(0.95),
                "maxOutputTokens": self.config["max_tokens"].as_u64().unwrap_or(1024),
            },
            "safetySettings": [
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_ONLY_HIGH"
                },
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_ONLY_HIGH"
                },
                {
                    "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                    "threshold": "BLOCK_ONLY_HIGH"
                },
                {
                    "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                    "threshold": "BLOCK_ONLY_HIGH"
                }
            ]
        });

        debug!("Gemini request body: {:?}", request_body);

        Ok(client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent",
                model
            ))
            .header("Content-Type", "application/json")
            .header("x-goog-api-key", api_key)
            .json(&request_body))
    }
    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Parsing Gemini response: {}", response_text);

        if let Ok(response_json) = serde_json::from_str::<Value>(response_text) {
            debug!("Parsed Gemini response JSON: {:?}", response_json);

            let mut result = String::new();

            // Handle both single response and array of responses
            let candidates = if response_json.is_array() {
                response_json.as_array().unwrap()
            } else {
                std::slice::from_ref(&response_json)
            };

            for chunk in candidates {
                if let Some(candidates) = chunk["candidates"].as_array() {
                    for candidate in candidates {
                        if let Some(content) = candidate["content"].as_object() {
                            if let Some(parts) = content["parts"].as_array() {
                                for part in parts {
                                    if let Some(text) = part["text"].as_str() {
                                        result.push_str(text);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !result.is_empty() {
                return Ok(result);
            }

            warn!("No text content found in Gemini response");
            return Ok(String::new());
        }

        warn!("Gemini response is not valid JSON, returning empty string");
        Ok(String::new())
    }

    fn stream_response(
        &self,
        response: reqwest::Response,
    ) -> Pin<Box<dyn Stream<Item = Result<String, LLMServiceError>> + Send>> {
        let self_clone = Arc::new(self.clone());

        Box::pin(
            stream::unfold(
                (response, String::new()),
                move |(mut response, mut buffer)| {
                    let self_clone = Arc::clone(&self_clone);
                    async move {
                        match response.chunk().await {
                            Ok(Some(chunk)) => {
                                let text = String::from_utf8_lossy(&chunk);
                                debug!("Received chunk: {}", text);
                                buffer.push_str(&text);

                                let mut result = String::new();
                                let mut valid_json_end = 0;

                                while let Ok(json) =
                                    serde_json::from_str::<Value>(&buffer[valid_json_end..])
                                {
                                    if let Ok(parsed) = self_clone.parse_response(&json.to_string())
                                    {
                                        result.push_str(&parsed);
                                    }
                                    valid_json_end = buffer.len();
                                }

                                buffer = buffer[valid_json_end..].to_string();

                                if !result.is_empty() {
                                    debug!("Streaming content: {}", result);
                                    Some((Ok(result), (response, buffer)))
                                } else {
                                    Some((Ok(String::new()), (response, buffer)))
                                }
                            }
                            Ok(None) => {
                                if !buffer.is_empty() {
                                    if let Ok(parsed) = self_clone.parse_response(&buffer) {
                                        Some((Ok(parsed), (response, String::new())))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Err(e) => {
                                error!("Error in stream_response: {:?}", e);
                                Some((
                                    Err(LLMServiceError(AppError::ExternalServiceError(format!(
                                        "Error in Gemini stream response: {}",
                                        e
                                    )))),
                                    (response, buffer),
                                ))
                            }
                        }
                    }
                },
            )
            .filter(|result| futures::future::ready(!matches!(result, Ok(ref s) if s.is_empty()))),
        )
    }
}

impl Clone for GeminiProvider {
    fn clone(&self) -> Self {
        GeminiProvider {
            config: self.config.clone(),
        }
    }
}

// Default configuration for Gemini provider
pub fn default_config() -> Value {
    serde_json::json!({
        "model": "gemini-pro",
        "temperature": 0.7,
        "top_k": 40,
        "top_p": 0.95,
        "max_tokens": 1024,
        "stream": true  // Explicitly enable streaming
    })
}
