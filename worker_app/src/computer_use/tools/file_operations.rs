use crate::computer_use::base::{ToolExecution, ToolInfo, ToolResult};
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextEditorRequest {
    pub command: String,
    pub path: Option<String>,
    pub text: Option<String>,
    pub file_text: Option<String>,
    pub pattern: Option<String>,
    pub replacement: Option<String>,
}

#[derive(Debug)]
pub struct FileOperations;

impl ToolInfo for FileOperations {
    fn name(&self) -> &'static str {
        "file_operations"
    }

    fn description(&self) -> &'static str {
        "Create and edit text files"
    }

    fn parameters(&self) -> Vec<(&'static str, bool)> {
        vec![
            ("command", true),
            ("path", true),
            ("text", false),
            ("file_text", false),
            ("pattern", false),
            ("replacement", false),
        ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl ToolExecution for FileOperations {
    async fn execute(
        &self,
        params: Value,
    ) -> Result<ToolResult, Box<dyn std::error::Error + Send + Sync>> {
        let request: TextEditorRequest = serde_json::from_value(params)?;
        info!("Received text editor request: {:?}", request);

        match request.command.as_str() {
            "create" => {
                if let Some(path) = request.path {
                    info!("Creating file at path: {}", path);

                    // Create parent directories if they don't exist
                    if let Some(parent) = Path::new(&path).parent() {
                        info!("Creating parent directories: {:?}", parent);
                        fs::create_dir_all(parent)?;
                    }

                    // Get content from either text or file_text
                    let content = request
                        .text
                        .or(request.file_text)
                        .ok_or("No content provided")?;

                    info!("Writing content to file, content length: {}", content.len());

                    // Write content exactly as received, no processing
                    fs::write(&path, content)?;

                    Ok(ToolResult {
                        success: true,
                        output: format!("File created successfully: {}", path),
                        error: None,
                    })
                } else {
                    Ok(ToolResult {
                        success: false,
                        output: String::from("No path provided"),
                        error: Some(String::from("Missing path parameter")),
                    })
                }
            }
            _ => Ok(ToolResult {
                success: false,
                output: format!("Unknown command: {}", request.command),
                error: Some(format!("Unsupported command: {}", request.command)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_file() {
        let tool = FileOperations;
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let result = tool
            .execute(json!({
                "command": "create",
                "path": test_file.to_str().unwrap(),
                "text": "test content"
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(test_file.exists());
        assert_eq!(fs::read_to_string(test_file).unwrap(), "test content");
    }

    #[tokio::test]
    async fn test_missing_path() {
        let tool = FileOperations;

        let result = tool
            .execute(json!({
                "command": "create",
                "text": "test content"
            }))
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_missing_content() {
        let tool = FileOperations;
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let result = tool
            .execute(json!({
                "command": "create",
                "path": test_file.to_str().unwrap()
            }))
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }
}
