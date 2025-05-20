use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Represents a template for LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Vec<String>,
    pub configuration: Value,
    pub logo_url: String,
    pub key_instructions: Vec<String>,
    pub is_default: bool,
}

/// Represents a unified LLM configuration that combines provider, API key, and user config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedLLMConfig {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Vec<String>,
    pub configuration: Value,
    pub api_key: String,
    pub api_key_description: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Request to create a new unified LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnifiedLLMConfigRequest {
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Vec<String>,
    pub configuration: Value,
    pub api_key: String,
    pub api_key_description: String,
}

/// Response for a unified LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedLLMConfigResponse {
    pub id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Vec<String>,
    pub configuration: Value,
    pub api_key_id: Uuid,
    pub api_key_description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
