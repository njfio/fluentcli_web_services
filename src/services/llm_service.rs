use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use diesel::prelude::*;
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMChatMessage {
    pub role: String,
    pub content: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMApiRequest {
    model: String,
    messages: Vec<LLMChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMApiResponse {
    choices: Vec<LLMApiChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMApiChoice {
    message: LLMChatMessage,
}

pub struct LLMService;

impl LLMService {
    fn get_llm_api_key(provider_name: &str) -> Result<String, AppError> {
        let key_name = format!("{}_API_KEY", provider_name.to_uppercase());
        env::var(&key_name).map_err(|_| AppError::ConfigError(format!("{} not found", key_name)))
    }

    pub fn create_llm_provider(
        pool: &DbPool,
        llm_name: String,
        llm_provider_type: String,
        llm_api_endpoint: String,
        llm_supported_modalities: Value,
        llm_configuration: Value,
    ) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        let new_llm_provider = NewLLMProvider {
            name: llm_name,
            provider_type: llm_provider_type,
            api_endpoint: llm_api_endpoint,
            supported_modalities: llm_supported_modalities,
            configuration: llm_configuration,
        };

        diesel::insert_into(llm_providers)
            .values(&new_llm_provider)
            .get_result::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_providers(pool: &DbPool) -> Result<Vec<LLMProvider>, AppError> {
        use crate::schema::llm_providers::dsl::*;

        llm_providers
            .load::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_provider(pool: &DbPool, llm_provider_id: Uuid) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        llm_providers
            .find(llm_provider_id)
            .first::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub async fn llm_chat(
        pool: &DbPool,
        llm_provider_id: Uuid,
        llm_messages: Vec<LLMChatMessage>,
    ) -> Result<String, AppError> {
        info!("Starting chat with provider_id: {}", llm_provider_id);
        let llm_provider = Self::get_llm_provider(pool, llm_provider_id)?;
        info!("Using provider: {:?}", llm_provider);

        let client = Client::new();
        let llm_api_key = Self::get_llm_api_key(&llm_provider.name)?;
        debug!(
            "API key retrieved successfully: {}",
            llm_api_key.chars().take(5).collect::<String>() + "..."
        );

        let llm_api_request = LLMApiRequest {
            model: llm_provider.configuration["model"]
                .as_str()
                .unwrap_or("default")
                .to_string(),
            messages: llm_messages,
            stream: false,
        };
        debug!("Sending request to LLM API: {:?}", llm_api_request);

        let llm_api_endpoint = &llm_provider.api_endpoint;
        debug!("Using API endpoint: {}", llm_api_endpoint);

        let llm_response = client
            .post(llm_api_endpoint)
            .header("Authorization", format!("Bearer {}", llm_api_key))
            .json(&llm_api_request)
            .send()
            .await
            .map_err(|e| {
                AppError::ExternalServiceError(format!("Failed to send request: {}", e))
            })?;

        let llm_status = llm_response.status();
        debug!("Received response from LLM API with status: {}", llm_status);

        let llm_response_text = llm_response.text().await.map_err(|e| {
            AppError::ExternalServiceError(format!("Failed to read response: {}", e))
        })?;
        debug!("Response body: {}", llm_response_text);

        if !llm_status.is_success() {
            return Err(AppError::ExternalServiceError(format!(
                "LLM API error: Status {}, Body: {}",
                llm_status, llm_response_text
            )));
        }

        let llm_api_response: LLMApiResponse =
            serde_json::from_str(&llm_response_text).map_err(|e| {
                AppError::ExternalServiceError(format!("Failed to parse response: {}", e))
            })?;

        info!("Parsed LLM response successfully");

        if llm_api_response.choices.is_empty() {
            return Err(AppError::ExternalServiceError(
                "LLM response contains no choices".to_string(),
            ));
        }

        serde_json::to_string(&llm_api_response.choices[0].message.content)
            .map_err(|e| AppError::SerializationError(e.to_string()))
    }

    pub async fn llm_stream_chat(
        pool: &DbPool,
        llm_provider_id: Uuid,
        llm_messages: Vec<LLMChatMessage>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, AppError>> + Send>>, AppError> {
        let llm_provider = Self::get_llm_provider(pool, llm_provider_id)?;
        let client = Client::new();
        let llm_api_key = Self::get_llm_api_key(&llm_provider.name)?;

        let llm_api_request = LLMApiRequest {
            model: llm_provider.configuration["model"]
                .as_str()
                .unwrap_or("default")
                .to_string(),
            messages: llm_messages,
            stream: true,
        };

        let llm_api_endpoint = &llm_provider.api_endpoint;
        debug!("Using API endpoint for streaming: {}", llm_api_endpoint);

        let llm_response = client
            .post(llm_api_endpoint)
            .header("Authorization", format!("Bearer {}", llm_api_key))
            .json(&llm_api_request)
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        let llm_stream = stream::unfold(llm_response, |mut response| async move {
            let chunk = response.chunk().await.ok()??;
            if chunk.is_empty() {
                None
            } else {
                let content = String::from_utf8_lossy(&chunk).into_owned();
                Some((Ok(content), response))
            }
        })
        .boxed();

        Ok(llm_stream)
    }
}
