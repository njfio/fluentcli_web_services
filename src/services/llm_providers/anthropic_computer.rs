use crate::error::AppError;
use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct AnthropicComputerProvider;

impl LLMProviderTrait for AnthropicComputerProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let client = Client::new();
        let model = config["model"].as_str().ok_or_else(|| {
            LLMServiceError(AppError::BadRequest(
                "Model not specified for Anthropic Computer provider".to_string(),
            ))
        })?;

        let filtered_messages: Vec<Value> = messages
            .iter()
            .filter(|msg| !msg.content.trim().is_empty())
            .map(|msg| {
                serde_json::json!({
                    "role": if msg.role == "user" { "user" } else { "assistant" },
                    "content": msg.content.trim()
                })
            })
            .collect();

        // Use tools from configuration if available, otherwise use defaults
        let tools = if let Some(config_tools) = config["tools"].as_array() {
            config_tools.clone()
        } else {
            vec![
                serde_json::json!({
                    "type": "computer_20241022",
                    "name": "computer",
                    "display_width_px": 1024,
                    "display_height_px": 768,
                    "display_number": 1
                }),
                serde_json::json!({
                    "type": "text_editor_20241022",
                    "name": "str_replace_editor"
                }),
                serde_json::json!({
                    "type": "bash_20241022",
                    "name": "bash"
                }),
            ]
        };

        let request_body = serde_json::json!({
            "model": model,
            "messages": filtered_messages,
            "max_tokens": config["max_tokens"].as_u64().unwrap_or(1024),
            "stream": true,
            "tools": tools
        });

        debug!("Anthropic Computer request body: {:?}", request_body);

        Ok(client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("anthropic-beta", "computer-use-2024-10-22")
            .header("Content-Type", "application/json")
            .json(&request_body))
    }

    fn parse_response(&self, response_text: &str) -> Result<String, LLMServiceError> {
        debug!("Anthropic Computer raw response: {}", response_text);
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
                                    // Handle both regular text responses and tool use responses
                                    if let Some(content) = json["delta"]["text"].as_str() {
                                        result.push_str(content);
                                    } else if let Some(tool_calls) =
                                        json["delta"]["tool_calls"].as_array()
                                    {
                                        // Format tool calls into a readable string
                                        for tool_call in tool_calls {
                                            if let (Some(tool_name), Some(args)) = (
                                                tool_call["name"].as_str(),
                                                tool_call["arguments"].as_str(),
                                            ) {
                                                result.push_str(&format!(
                                                    "Tool Call: {} with args: {}\n",
                                                    tool_name, args
                                                ));
                                            }
                                        }
                                    } else if let Some(error) = json["error"].as_object() {
                                        let error_message =
                                            error["message"].as_str().unwrap_or("Unknown error");
                                        return Some((
                                            Err(LLMServiceError(AppError::ExternalServiceError(
                                                format!(
                                                    "Anthropic Computer API error: {}",
                                                    error_message
                                                ),
                                            ))),
                                            response,
                                        ));
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
