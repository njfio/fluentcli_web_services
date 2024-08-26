use std::process::Command;

pub struct FluentCLIService;

impl FluentCLIService {
    pub fn execute_command(&self, command: &str) -> Result<String, AppError> {
        let output = Command::new("fluentcli")
            .args(command.split_whitespace())
            .output()
            .map_err(|e| AppError::FluentCLIError(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(AppError::FluentCLIError(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}