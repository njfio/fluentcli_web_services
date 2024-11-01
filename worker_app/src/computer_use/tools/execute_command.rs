use crate::computer_use::base::{ToolExecution, ToolInfo, ToolResult};
use async_trait::async_trait;
use log::info;
use serde_json::Value;
use std::any::Any;
use std::error::Error;
use std::process::Command;

#[derive(Debug)]
pub struct ExecuteCommand;

impl ToolInfo for ExecuteCommand {
    fn name(&self) -> &'static str {
        "execute_command"
    }

    fn description(&self) -> &'static str {
        "Execute a CLI command on the system"
    }

    fn parameters(&self) -> Vec<(&'static str, bool)> {
        vec![("command", true)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[async_trait]
impl ToolExecution for ExecuteCommand {
    async fn execute(&self, params: Value) -> Result<ToolResult, Box<dyn Error + Send + Sync>> {
        self.validate_params(&params)?;

        let command = params["command"].as_str().unwrap();
        info!("Executing command: {}", command);

        // Execute command directly like reference implementation
        let output = Command::new("sh").arg("-c").arg(command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(ToolResult {
            success: output.status.success(),
            output: format!("Command output:\nstdout: {}\nstderr: {}", stdout, stderr),
            error: if !output.status.success() {
                Some(format!("Command failed with status: {}", output.status))
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_execute_command() {
        let tool = ExecuteCommand;

        // Test successful command
        let result = tool
            .execute(json!({
                "command": "echo 'test'"
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("test"));
        assert!(result.error.is_none());

        // Test failed command
        let result = tool
            .execute(json!({
                "command": "nonexistent_command"
            }))
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_mkdir_command() {
        use std::fs;
        use tempfile::tempdir;

        let tool = ExecuteCommand;
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_dir");

        // Test mkdir command
        let result = tool
            .execute(json!({
                "command": format!("mkdir -p {}", test_dir.display())
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
