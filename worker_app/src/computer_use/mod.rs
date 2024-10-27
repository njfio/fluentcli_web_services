use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolConfig {
    pub display_width_px: u32,
    pub display_height_px: u32,
    pub display_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolRequest {
    pub action: String,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub keys: Option<Vec<String>>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextEditorRequest {
    pub text: String,
    pub operation: String,
    pub pattern: Option<String>,
    pub replacement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BashRequest {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub screenshot: Option<String>,
}

pub struct ComputerUseService {
    config: ComputerToolConfig,
}

impl ComputerUseService {
    pub fn new(config: ComputerToolConfig) -> Self {
        Self { config }
    }

    pub async fn handle_computer_request(&self, request: ComputerToolRequest) -> ToolResponse {
        // TODO: Implement computer interaction using xdotool or similar
        ToolResponse {
            success: true,
            output: "Computer action executed".to_string(),
            error: None,
            screenshot: None,
        }
    }

    pub async fn handle_text_editor_request(&self, request: TextEditorRequest) -> ToolResponse {
        match request.operation.as_str() {
            "replace" => {
                if let (Some(pattern), Some(replacement)) = (request.pattern, request.replacement) {
                    let modified_text = request.text.replace(&pattern, &replacement);
                    ToolResponse {
                        success: true,
                        output: modified_text,
                        error: None,
                        screenshot: None,
                    }
                } else {
                    ToolResponse {
                        success: false,
                        output: String::new(),
                        error: Some(
                            "Pattern and replacement required for replace operation".to_string(),
                        ),
                        screenshot: None,
                    }
                }
            }
            _ => ToolResponse {
                success: false,
                output: String::new(),
                error: Some(format!("Unsupported operation: {}", request.operation)),
                screenshot: None,
            },
        }
    }

    pub async fn handle_bash_request(&self, request: BashRequest) -> ToolResponse {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&request.command)
            .args(&request.args)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                ToolResponse {
                    success: output.status.success(),
                    output: stdout,
                    error: if stderr.is_empty() {
                        None
                    } else {
                        Some(stderr)
                    },
                    screenshot: None,
                }
            }
            Err(e) => ToolResponse {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to execute bash command: {}", e)),
                screenshot: None,
            },
        }
    }
}
