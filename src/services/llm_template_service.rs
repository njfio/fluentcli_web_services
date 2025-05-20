use crate::db::DbPool;
use crate::error::AppError;
use crate::models::api_key::{ApiKey, NewApiKey};
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use crate::models::llm_template::{LLMTemplate, UnifiedLLMConfig, UnifiedLLMConfigResponse};
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::services::api_key_service::ApiKeyService;
use crate::services::llm_provider::LLMProviderService;
use log::{debug, error, info};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

pub struct LLMTemplateService;

impl LLMTemplateService {
    /// Get all available LLM templates
    pub fn get_templates() -> Vec<LLMTemplate> {
        vec![
            // OpenAI GPT-4
            LLMTemplate {
                id: "openai-gpt4".to_string(),
                name: "OpenAI GPT-4".to_string(),
                description: "OpenAI's most advanced model for text generation and understanding".to_string(),
                provider_type: "gpt".to_string(),
                api_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
                supported_modalities: vec!["text".to_string()],
                configuration: json!({
                    "model": "gpt-4o",
                    "temperature": 0.7,
                    "max_tokens": 1024,
                    "top_p": 1,
                    "frequency_penalty": 0,
                    "presence_penalty": 0
                }),
                logo_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/0/04/ChatGPT_logo.svg/1024px-ChatGPT_logo.svg.png".to_string(),
                key_instructions: vec![
                    "Go to https://platform.openai.com/account/api-keys".to_string(),
                    "Sign in or create an account".to_string(),
                    "Click \"Create new secret key\"".to_string(),
                    "Copy the key (you won't be able to see it again)".to_string(),
                ],
                is_default: true,
            },
            // Anthropic Claude
            LLMTemplate {
                id: "anthropic-claude".to_string(),
                name: "Anthropic Claude".to_string(),
                description: "Claude is a family of AI assistants created by Anthropic".to_string(),
                provider_type: "claude".to_string(),
                api_endpoint: "https://api.anthropic.com/v1/messages".to_string(),
                supported_modalities: vec!["text".to_string()],
                configuration: json!({
                    "model": "claude-3-opus-20240229",
                    "temperature": 0.7,
                    "max_tokens": 1024,
                    "top_p": 1
                }),
                logo_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Anthropic_logo.svg/1200px-Anthropic_logo.svg.png".to_string(),
                key_instructions: vec![
                    "Go to https://console.anthropic.com/".to_string(),
                    "Sign in or create an account".to_string(),
                    "Navigate to API Keys".to_string(),
                    "Click \"Create Key\"".to_string(),
                ],
                is_default: true,
            },
            // Perplexity
            LLMTemplate {
                id: "perplexity".to_string(),
                name: "Perplexity".to_string(),
                description: "Perplexity AI offers online LLMs with real-time information access".to_string(),
                provider_type: "perplexity".to_string(),
                api_endpoint: "https://api.perplexity.ai/chat/completions".to_string(),
                supported_modalities: vec!["text".to_string()],
                configuration: json!({
                    "model": "llama-3.1-sonar-huge-128k-online",
                    "max_tokens": 1024,
                    "temperature": 0.7
                }),
                logo_url: "https://cdn.icon-icons.com/icons2/3914/PNG/512/perplexity_logo_icon_248863.png".to_string(),
                key_instructions: vec![
                    "Go to https://www.perplexity.ai/settings/api".to_string(),
                    "Sign in or create an account".to_string(),
                    "Generate a new API key".to_string(),
                    "Copy the key".to_string(),
                ],
                is_default: true,
            },
            // Gemini
            LLMTemplate {
                id: "gemini".to_string(),
                name: "Google Gemini".to_string(),
                description: "Google's most capable AI model for text, code, and multimodal tasks".to_string(),
                provider_type: "gemini".to_string(),
                api_endpoint: "https://generativelanguage.googleapis.com/v1beta/models".to_string(),
                supported_modalities: vec!["text".to_string()],
                configuration: json!({
                    "model": "gemini-1.5-pro",
                    "temperature": 0.7,
                    "top_k": 40,
                    "top_p": 0.95,
                    "max_tokens": 1024
                }),
                logo_url: "https://storage.googleapis.com/gweb-uniblog-publish-prod/images/gemini_1.max-1000x1000.png".to_string(),
                key_instructions: vec![
                    "Go to https://ai.google.dev/".to_string(),
                    "Sign in with your Google account".to_string(),
                    "Navigate to API Keys".to_string(),
                    "Create a new API key".to_string(),
                ],
                is_default: true,
            },
            // Mistral
            LLMTemplate {
                id: "mistral".to_string(),
                name: "Mistral AI".to_string(),
                description: "Mistral AI offers powerful and efficient language models".to_string(),
                provider_type: "mistral".to_string(),
                api_endpoint: "https://api.mistral.ai/v1/chat/completions".to_string(),
                supported_modalities: vec!["text".to_string()],
                configuration: json!({
                    "model": "mistral-large-latest",
                    "temperature": 0.7,
                    "max_tokens": 1024,
                    "top_p": 0.95
                }),
                logo_url: "https://mistral.ai/images/logo.svg".to_string(),
                key_instructions: vec![
                    "Go to https://console.mistral.ai/".to_string(),
                    "Sign in or create an account".to_string(),
                    "Navigate to API Keys".to_string(),
                    "Create a new API key".to_string(),
                ],
                is_default: true,
            },
        ]
    }

    /// Get a template by ID
    pub fn get_template_by_id(template_id: &str) -> Option<LLMTemplate> {
        Self::get_templates()
            .into_iter()
            .find(|template| template.id == template_id)
    }

    /// Create a unified LLM configuration
    pub fn create_unified_config(
        pool: &DbPool,
        user_id: Uuid,
        config: UnifiedLLMConfig,
    ) -> Result<UnifiedLLMConfigResponse, AppError> {
        info!("Creating unified LLM configuration for user: {}", user_id);
        
        // Create API key
        let api_key = Self::create_api_key(pool, user_id, &config)?;
        debug!("Created API key with ID: {}", api_key.id);
        
        // Create LLM provider
        let provider = Self::create_provider(pool, user_id, &config)?;
        debug!("Created LLM provider with ID: {}", provider.id);
        
        // Create user LLM config
        let user_config = Self::create_user_config(
            pool,
            user_id,
            provider.id,
            api_key.id,
            config.name.clone(),
        )?;
        debug!("Created user LLM config with ID: {}", user_config.id);
        
        // Return unified response
        Ok(UnifiedLLMConfigResponse {
            id: user_config.id,
            name: config.name,
            provider_type: config.provider_type,
            api_endpoint: config.api_endpoint,
            supported_modalities: config.supported_modalities,
            configuration: config.configuration,
            api_key_id: api_key.id,
            api_key_description: api_key.description.unwrap_or_default(),
            created_at: user_config.created_at.and_utc(),
            updated_at: user_config.updated_at.and_utc(),
        })
    }

    /// Get all unified LLM configurations for a user
    pub fn get_unified_configs(
        pool: &DbPool,
        user_id: Uuid,
    ) -> Result<Vec<UnifiedLLMConfigResponse>, AppError> {
        info!("Getting unified LLM configurations for user: {}", user_id);
        
        // Get all user LLM configs
        let user_configs = crate::services::llm_provider::LLMProviderService::list_user_llm_configs(
            pool, user_id,
        )?;
        
        let mut result = Vec::new();
        
        for user_config in user_configs {
            // Get provider
            let provider = LLMProviderService::get_llm_provider(pool, user_config.provider_id)?;
            
            // Get API key
            let api_key = match ApiKeyService::get_api_key_by_id(pool, user_config.api_key_id)? {
                Some(key) => key,
                None => {
                    error!("API key not found for user config: {}", user_config.id);
                    continue;
                }
            };
            
            // Create unified response
            result.push(UnifiedLLMConfigResponse {
                id: user_config.id,
                name: user_config.description.unwrap_or_else(|| provider.name.clone()),
                provider_type: provider.provider_type,
                api_endpoint: provider.api_endpoint,
                supported_modalities: Self::json_to_vec_string(&provider.supported_modalities),
                configuration: provider.configuration,
                api_key_id: api_key.id,
                api_key_description: api_key.description.unwrap_or_default(),
                created_at: user_config.created_at.and_utc(),
                updated_at: user_config.updated_at.and_utc(),
            });
        }
        
        Ok(result)
    }

    /// Delete a unified LLM configuration
    pub fn delete_unified_config(
        pool: &DbPool,
        user_id: Uuid,
        config_id: Uuid,
    ) -> Result<(), AppError> {
        info!("Deleting unified LLM configuration: {}", config_id);
        
        // Get the user LLM config
        let user_config = match crate::services::llm_provider::LLMProviderService::get_user_llm_config(
            pool, config_id,
        )? {
            Some(config) => config,
            None => return Err(AppError::NotFoundError("User LLM config not found".to_string())),
        };
        
        // Check if the user owns this config
        if user_config.user_id != user_id {
            return Err(AppError::Unauthorized);
        }
        
        // Delete the user LLM config
        crate::services::llm_provider::LLMProviderService::delete_user_llm_config(
            pool, config_id,
        )?;
        
        // Delete the provider
        crate::services::llm_provider::LLMProviderService::delete_llm_provider(
            pool, user_config.provider_id,
        )?;
        
        // Delete the API key
        ApiKeyService::delete_api_key(pool, user_config.api_key_id)?;
        
        Ok(())
    }

    // Helper methods
    fn create_api_key(
        pool: &DbPool,
        user_id: Uuid,
        config: &UnifiedLLMConfig,
    ) -> Result<ApiKey, AppError> {
        ApiKeyService::create_api_key(
            pool,
            user_id,
            config.api_key.clone(),
            Some(config.api_key_description.clone()),
            None, // No expiration
        )
    }

    fn create_provider(
        pool: &DbPool,
        user_id: Uuid,
        config: &UnifiedLLMConfig,
    ) -> Result<LLMProvider, AppError> {
        let new_provider = NewLLMProvider {
            user_id,
            name: config.name.clone(),
            provider_type: config.provider_type.clone(),
            api_endpoint: config.api_endpoint.clone(),
            supported_modalities: json!(config.supported_modalities),
            configuration: config.configuration.clone(),
        };
        
        LLMProviderService::create_llm_provider(pool, new_provider)
    }

    fn create_user_config(
        pool: &DbPool,
        user_id: Uuid,
        provider_id: Uuid,
        api_key_id: Uuid,
        description: String,
    ) -> Result<UserLLMConfig, AppError> {
        let new_config = NewUserLLMConfig {
            user_id,
            provider_id,
            api_key_id,
            description: Some(description),
        };
        
        LLMProviderService::create_user_llm_config(pool, new_config)
    }

    fn json_to_vec_string(json_value: &Value) -> Vec<String> {
        match json_value {
            Value::Array(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            Value::String(s) => vec![s.clone()],
            _ => Vec::new(),
        }
    }
}
