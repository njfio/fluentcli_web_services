use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, warn};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct CohereProvider;

impl LLMProviderTrait for CohereProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().ok_or_else(|| {
            LLMServiceError(AppError::BadRequest(
                "Model not specified for Cohere provider".to_string(),
            ))
        })?;

        let chat_history: Vec<Value> = messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "role": msg.role,
                    "message": msg.content
                })
            })
            .collect();

        let request_body = serde_json::json!({
            "model": model,
            "chat_history": chat_history,
            "message": messages.last().map(|msg| msg.content.clone()).unwrap_or_default(),
            "stream": true
        });

        debug!("Cohere request body: {:?}", request_body);
        debug!(
            "Cohere API key (first 10 chars): {}",
            &api_key[..10.min(api_key.len())]
        );

        Ok(client
            .post("https://api.cohere.ai/v1/chat")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Cohere raw response: {}", response_text);
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
                            if let Ok(json) = serde_json::from_str::<Value>(line) {
                                if let Some(text) = json["text"].as_str() {
                                    result.push_str(text);
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
