use crate::error::AppError;
use crate::services::llm_service::{
    ContentBlock, LLMChatMessage, LLMProviderTrait, LLMServiceError,
};
use chrono::Local;
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, info};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct AnthropicComputerProvider;

#[derive(Debug)]
enum StreamState {
    Normal(String),
    CollectingJson {
        id: String,
        name: String,
        json_str: String,
        text_buffer: String, // Buffer for text while collecting JSON
    },
}

impl StreamState {
    fn append_text(&mut self, text: &str) {
        match self {
            StreamState::Normal(buffer) => buffer.push_str(text),
            StreamState::CollectingJson { text_buffer, .. } => text_buffer.push_str(text),
        }
    }

    fn append_json(&mut self, json: &str) {
        if let StreamState::CollectingJson { json_str, .. } = self {
            json_str.push_str(json);
        }
    }

    fn take_text(&mut self) -> String {
        match self {
            StreamState::Normal(buffer) => std::mem::take(buffer),
            StreamState::CollectingJson { text_buffer, .. } => std::mem::take(text_buffer),
        }
    }

    fn take_json(&mut self) -> Option<(String, String, String)> {
        if let StreamState::CollectingJson {
            id, name, json_str, ..
        } = self
        {
            Some((
                std::mem::take(id),
                std::mem::take(name),
                std::mem::take(json_str),
            ))
        } else {
            None
        }
    }
}

async fn execute_tool(id: &str, name: &str, json_str: &str) -> Result<String, String> {
    // Parse the input JSON but don't modify it
    let tool_input = serde_json::from_str::<Value>(json_str)
        .map_err(|e| format!("Failed to parse tool input: {}", e))?;

    let worker_url = std::env::var("WORKER_COMPUTER_ADDRESS")
        .unwrap_or_else(|_| "http://worker:8081/computer-use".to_string());

    let client = Client::new();

    // Map tool names to endpoints but keep the original input
    let endpoint = match name {
        "bash" => format!("{}/bash_20241022", worker_url),
        "str_replace_editor" => format!("{}/text_editor_20241022", worker_url),
        "computer" => format!("{}/computer_20241022", worker_url),
        _ => return Err(format!("Unknown tool: {}", name)),
    };

    info!("Executing tool {} with input: {}", name, json_str);

    // Send the exact input JSON to the worker
    let response = client
        .post(&endpoint)
        .json(&tool_input)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to worker: {}", e))?;

    let response_status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;

    if !response_status.is_success() {
        return Ok(serde_json::json!({
            "type": "tool_result",
            "tool_use_id": id,
            "content": [{
                "type": "text",
                "text": format!("Error: {}", response_text)
            }],
            "is_error": true
        })
        .to_string());
    }

    // Return the exact response from the worker
    Ok(serde_json::json!({
        "type": "tool_result",
        "tool_use_id": id,
        "content": [{
            "type": "text",
            "text": response_text
        }]
    })
    .to_string())
}

impl LLMProviderTrait for AnthropicComputerProvider {
    fn prepare_request(
        &self,
        messages: &[LLMChatMessage],
        config: &Value,
        api_key: &str,
    ) -> Result<reqwest::RequestBuilder, LLMServiceError> {
        let model = config["model"]
            .as_str()
            .ok_or_else(|| {
                LLMServiceError(AppError::BadRequest(
                    "Model not specified for Anthropic Computer provider".to_string(),
                ))
            })?
            .to_string();

        let filtered_messages: Vec<Value> = messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "role": if msg.role == "user" { "user" } else { "assistant" },
                    "content": msg.content
                })
            })
            .collect();

        let tools = vec![
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
        ];

        let now = Local::now();
        let system_prompt = format!(
            "You are Claude, an AI assistant with access to a virtual computer. You can use tools to interact with this computer:

1. The 'computer' tool lets you control mouse and keyboard
2. The 'str_replace_editor' tool lets you create and edit text files
3. The 'bash' tool lets you run shell commands

Important guidelines:
- When using tools, wait for each tool call to complete before making another
- For file operations, use absolute paths starting with /repo/
- The current date is {}
- GUI applications may take time to appear - be patient and verify with screenshots
- For large command outputs, save to a file and use grep or the editor to examine
- When downloading files, use curl instead of wget

Remember to:
- Create directories before writing files
- Use full paths for file operations
- Handle errors gracefully
- Verify results of tool operations before proceeding",
            now.format("%A, %B %-d, %Y")
        );

        let max_tokens = config["max_tokens"].as_u64().unwrap_or(4096);

        let request_body = serde_json::json!({
            "model": model,
            "messages": filtered_messages,
            "max_tokens": max_tokens,
            "stream": true,
            "tools": tools,
            "system": system_prompt
        });

        debug!("Anthropic Computer request body: {}", request_body);

        let client = Client::new();
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
            stream::unfold((response, StreamState::Normal(String::new())), move |(mut response, mut state)| {
                async move {
                    match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let text = String::from_utf8_lossy(&chunk);
                            let mut result = String::new();

                            for line in text.lines() {
                                if line.starts_with("data: ") {
                                    let json_str = line.trim_start_matches("data: ");
                                    if json_str == "[DONE]" {
                                        result.push_str(&state.take_text());
                                        return Some((Ok(result), (response, state)));
                                    }
                                    if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                                        match json["type"].as_str() {
                                            Some("content_block_delta") => {
                                                if let Some(delta) = json["delta"].as_object() {
                                                    if let Some(text) = delta.get("text").and_then(|v| v.as_str()) {
                                                        state.append_text(text);
                                                    } else if let Some(json) = delta.get("partial_json").and_then(|v| v.as_str()) {
                                                        state.append_json(json);
                                                    }
                                                }
                                            }
                                            Some("content_block_start") => {
                                                if let Some(content_block) = json["content_block"].as_object() {
                                                    if content_block["type"] == "tool_use" {
                                                        if let (Some(id), Some(name)) = (content_block["id"].as_str(), content_block["name"].as_str()) {
                                                            let text = state.take_text();
                                                            if !text.is_empty() {
                                                                result.push_str(&text);
                                                            }
                                                            state = StreamState::CollectingJson {
                                                                id: id.to_string(),
                                                                name: name.to_string(),
                                                                json_str: String::new(),
                                                                text_buffer: String::new(),
                                                            };
                                                        }
                                                    }
                                                }
                                            }
                                            Some("content_block_stop") => {
                                                if let Some((id, name, json_str)) = state.take_json() {
                                                    result.push_str(&execute_tool(&id, &name, &json_str).await.unwrap_or_else(|e| {
                                                        let error_result = serde_json::json!({
                                                            "type": "tool_result",
                                                            "tool_use_id": id,
                                                            "content": [{
                                                                "type": "text",
                                                                "text": format!("Error executing tool: {}", e)
                                                            }],
                                                            "is_error": true
                                                        });
                                                        error_result.to_string()
                                                    }));
                                                    state = StreamState::Normal(String::new());
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            if !result.is_empty() {
                                Some((Ok(result), (response, state)))
                            } else {
                                Some((Ok(String::new()), (response, state)))
                            }
                        }
                        Ok(None) => None,
                        Err(e) => {
                            error!("Error in stream_response: {:?}", e);
                            Some((
                                Err(LLMServiceError(AppError::ExternalServiceError(
                                    e.to_string(),
                                ))),
                                (response, state),
                            ))
                        }
                    }
                }
            })
            .filter(|result| futures::future::ready(!matches!(result, Ok(ref s) if s.is_empty()))),
        )
    }
}
