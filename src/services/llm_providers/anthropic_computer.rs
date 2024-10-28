use crate::error::AppError;
use crate::services::llm_service::{
    ContentBlock, LLMChatMessage, LLMProviderTrait, LLMServiceError,
};
use chrono::Local;
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, info, warn};
use reqwest::Client;
use serde_json::Value;
use std::pin::Pin;

pub struct AnthropicComputerProvider;

#[derive(Debug)]
struct ToolState {
    id: String,
    name: String,
    json_buffer: String,
    complete: bool,
}

async fn execute_tool(state: &ToolState, mut tool_input: Value) -> Result<String, String> {
    let client = Client::new();
    let endpoint = match state.name.as_str() {
        "str_replace_editor" => {
            // Convert file_text to text for the worker
            if let Some(obj) = tool_input.as_object_mut() {
                if let Some(file_text) = obj.remove("file_text") {
                    obj.insert("text".to_string(), file_text);
                }
            }
            "text-editor"
        }
        "bash" => "bash",
        "computer" => "computer",
        _ => return Err(format!("Unknown tool: {}", state.name)),
    };

    let url = format!("http://worker:8081/computer-use/{}", endpoint);
    info!(
        "Sending request to worker: {} with input: {}",
        url, tool_input
    );

    match client.post(&url).json(&tool_input).send().await {
        Ok(response) => {
            info!(
                "Received response from worker with status: {}",
                response.status()
            );
            match response.json::<Value>().await {
                Ok(tool_output) => {
                    info!("Worker response: {}", tool_output);
                    let tool_result = serde_json::json!({
                        "type": "tool_result",
                        "tool_use_id": state.id,
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&tool_output).unwrap_or_else(|_| tool_output.to_string())
                        }]
                    });
                    Ok(tool_result.to_string())
                }
                Err(e) => Err(format!("Failed to parse worker response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to send request to worker: {}", e)),
    }
}

fn is_valid_json(json_str: &str) -> bool {
    serde_json::from_str::<Value>(json_str).is_ok()
}

async fn create_repo_directory() -> Result<(), String> {
    let client = Client::new();
    match client
        .post("http://worker:8081/computer-use/bash")
        .json(&serde_json::json!({
            "command": "mkdir -p /repo"
        }))
        .send()
        .await
    {
        Ok(_) => {
            info!("Successfully created /repo directory");
            Ok(())
        }
        Err(e) => {
            error!("Failed to create /repo directory: {}", e);
            Err(e.to_string())
        }
    }
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
            .filter(|msg| !msg.content.trim().is_empty())
            .map(|msg| {
                serde_json::json!({
                    "role": if msg.role == "user" { "user" } else { "assistant" },
                    "content": msg.content.trim()
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
            stream::unfold((response, None::<ToolState>, String::new()), move |(mut response, mut tool_state, mut text_buffer)| {
                async move {
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
                                        // If we have a pending tool state with valid JSON, execute it
                                        if let Some(ref mut state) = tool_state {
                                            if !state.complete && is_valid_json(&state.json_buffer) {
                                                info!("Executing pending tool use on stream end");
                                                if let Ok(tool_input) = serde_json::from_str::<Value>(&state.json_buffer) {
                                                    result.push_str(&execute_tool(state, tool_input).await.unwrap_or_else(|e| {
                                                        let error_result = serde_json::json!({
                                                            "type": "tool_result",
                                                            "tool_use_id": state.id,
                                                            "content": [{
                                                                "type": "text",
                                                                "text": format!("Error executing tool: {}", e)
                                                            }],
                                                            "is_error": true
                                                        });
                                                        error_result.to_string()
                                                    }));
                                                }
                                            }
                                        }
                                        return Some((Ok(result), (response, tool_state, text_buffer)));
                                    }
                                    if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                                        match json["type"].as_str() {
                                            Some("content_block_delta") => {
                                                if let Some(delta) = json["delta"].as_object() {
                                                    if delta["type"] == "text_delta" {
                                                        if let Some(text) = delta["text"].as_str() {
                                                            text_buffer.push_str(text);
                                                            result.push_str(text);
                                                        }
                                                    } else if delta["type"] == "input_json_delta" {
                                                        if let Some(partial_json) = delta["partial_json"].as_str() {
                                                            if let Some(ref mut state) = tool_state {
                                                                state.json_buffer.push_str(partial_json);
                                                                debug!("Current JSON buffer: {}", state.json_buffer);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some("content_block_start") => {
                                                if let Some(content_block) = json["content_block"].as_object() {
                                                    if content_block["type"] == "tool_use" {
                                                        if let (Some(id), Some(name)) = (content_block["id"].as_str(), content_block["name"].as_str()) {
                                                            info!("Tool use block started: {} {}", name, id);
                                                            tool_state = Some(ToolState {
                                                                id: id.to_string(),
                                                                name: name.to_string(),
                                                                json_buffer: String::new(),
                                                                complete: false,
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                            Some("content_block_stop") => {
                                                if let Some(ref mut state) = tool_state {
                                                    info!("Tool use block stopped: {} {}", state.name, state.id);
                                                    info!("Tool input: {}", state.json_buffer);

                                                    if is_valid_json(&state.json_buffer) {
                                                        if let Ok(tool_input) = serde_json::from_str::<Value>(&state.json_buffer) {
                                                            // Create /repo directory if needed
                                                            if let Some(path) = tool_input["path"].as_str() {
                                                                if path.starts_with("/repo") {
                                                                    if let Err(e) = create_repo_directory().await {
                                                                        error!("Failed to create /repo directory: {}", e);
                                                                    }
                                                                }
                                                            }

                                                            result.push_str(&execute_tool(state, tool_input).await.unwrap_or_else(|e| {
                                                                let error_result = serde_json::json!({
                                                                    "type": "tool_result",
                                                                    "tool_use_id": state.id,
                                                                    "content": [{
                                                                        "type": "text",
                                                                        "text": format!("Error executing tool: {}", e)
                                                                    }],
                                                                    "is_error": true
                                                                });
                                                                error_result.to_string()
                                                            }));
                                                        }
                                                    } else {
                                                        warn!("Invalid JSON in tool use block: {}", state.json_buffer);
                                                    }
                                                    state.complete = true;
                                                }
                                            }
                                            Some("message_delta") => {
                                                if let Some(stop_reason) = json["delta"]["stop_reason"].as_str() {
                                                    debug!("Message stopped: {}", stop_reason);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            if !result.is_empty() {
                                Some((Ok(result), (response, tool_state, text_buffer)))
                            } else {
                                Some((Ok(String::new()), (response, tool_state, text_buffer)))
                            }
                        }
                        Ok(None) => None,
                        Err(e) => {
                            error!("Error in stream_response: {:?}", e);
                            Some((
                                Err(LLMServiceError(AppError::ExternalServiceError(
                                    e.to_string(),
                                ))),
                                (response, tool_state, text_buffer),
                            ))
                        }
                    }
                }
            })
            .filter(|result| futures::future::ready(!matches!(result, Ok(ref s) if s.is_empty()))),
        )
    }
}
