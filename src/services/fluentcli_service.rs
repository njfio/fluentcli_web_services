use crate::error::AppError;
use crate::models::fluentcli::{CommandRequest, CommandResult};
use log::{debug, info};
use reqwest;
use uuid::Uuid;

const WORKER_ADDRESS: &str = "http://worker:8080"; // Adjust this to match your Docker setup

pub struct FluentCLIService;

impl FluentCLIService {
    pub async fn execute_command(
        user_id: Uuid,
        command: CommandRequest,
    ) -> Result<CommandResult, AppError> {
        info!("Executing command for user_id: {}", user_id);
        debug!("Command request: {:?}", command);

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/execute", WORKER_ADDRESS))
            .json(&command)
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        let result: CommandResult = response
            .json()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        Ok(result)
    }
}
