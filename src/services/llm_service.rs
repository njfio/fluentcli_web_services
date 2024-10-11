use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use diesel::prelude::*;
use futures::stream::{self, Stream, StreamExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIChoice {
    message: ChatMessage,
}

pub struct LLMService;

impl LLMService {
    fn get_openai_api_key() -> Result<String, AppError> {
        env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::ConfigError("OpenAI API key not found".to_string()))
    }

    pub fn create_llm_provider(
        pool: &DbPool,
        provider_name: String,
        provider_api_endpoint: String,
    ) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        let new_provider = NewLLMProvider {
            name: provider_name,
            api_endpoint: provider_api_endpoint,
        };

        diesel::insert_into(llm_providers)
            .values(&new_provider)
            .get_result::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_providers(pool: &DbPool) -> Result<Vec<LLMProvider>, AppError> {
        use crate::schema::llm_providers::dsl::*;

        llm_providers
            .load::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        llm_providers
            .find(provider_id)
            .first::<LLMProvider>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub async fn chat(
        pool: &DbPool,
        provider_id: Uuid,
        messages: Vec<ChatMessage>,
    ) -> Result<String, AppError> {
        info!("Starting chat with provider_id: {}", provider_id);
        let provider = Self::get_llm_provider(pool, provider_id)?;
        info!("Using provider: {:?}", provider);

        let client = Client::new();
        let api_key = Self::get_openai_api_key()?;
        debug!(
            "API key retrieved successfully: {}",
            api_key.chars().take(5).collect::<String>() + "..."
        );

        let openai_request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: messages.clone(),
            stream: false,
        };
        debug!("Sending request to OpenAI API: {:?}", openai_request);

        // Ensure the API endpoint is correct
        let api_endpoint = "https://api.openai.com/v1/chat/completions";
        debug!("Using API endpoint: {}", api_endpoint);

        let response = client
            .post(api_endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&openai_request)
            .send()
            .await
            .map_err(|e| {
                error!("Error sending request to OpenAI: {:?}", e);
                AppError::ExternalServiceError(format!("Failed to send request: {}", e))
            })?;

        let status = response.status();
        debug!("Received response from OpenAI API with status: {}", status);

        let response_text = response.text().await.map_err(|e| {
            error!("Error reading response body: {:?}", e);
            AppError::ExternalServiceError(format!("Failed to read response: {}", e))
        })?;
        debug!("Response body: {}", response_text);

        if !status.is_success() {
            error!(
                "OpenAI API returned an error. Status: {}, Body: {}",
                status, response_text
            );
            return Err(AppError::ExternalServiceError(format!(
                "OpenAI API error: Status {}, Body: {}",
                status, response_text
            )));
        }

        let openai_response: OpenAIResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                error!("Error parsing OpenAI response: {:?}", e);
                AppError::ExternalServiceError(format!("Failed to parse response: {}", e))
            })?;

        info!("Parsed OpenAI response successfully");

        if openai_response.choices.is_empty() {
            error!("OpenAI response contains no choices");
            return Err(AppError::ExternalServiceError(
                "OpenAI response contains no choices".to_string(),
            ));
        }

        Ok(openai_response.choices[0].message.content.clone())
    }

    pub async fn stream_chat(
        pool: &DbPool,
        provider_id: Uuid,
        messages: Vec<ChatMessage>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, AppError>> + Send>>, AppError> {
        let provider = Self::get_llm_provider(pool, provider_id)?;
        let client = Client::new();
        let api_key = Self::get_openai_api_key()?;

        let openai_request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            stream: true,
        };

        // Ensure the API endpoint is correct
        let api_endpoint = "https://api.openai.com/v1/chat/completions";
        debug!("Using API endpoint for streaming: {}", api_endpoint);

        let response = client
            .post(api_endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&openai_request)
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        let stream = stream::unfold(response, |mut response| async move {
            let chunk = response.chunk().await.ok()??;
            if chunk.is_empty() {
                None
            } else {
                let content = String::from_utf8_lossy(&chunk).into_owned();
                Some((Ok(content), response))
            }
        })
        .boxed();

        Ok(stream)
    }
}
