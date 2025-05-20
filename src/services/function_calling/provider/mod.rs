pub mod adapter;
pub mod openai;
pub mod anthropic;

pub use adapter::{ProviderAdapter, ToolChoice};
pub use openai::OpenAIAdapter;
pub use anthropic::AnthropicAdapter;

use std::sync::Arc;
use crate::models::llm_provider::LLMProvider;

/// Creates a provider adapter based on the provider type
pub fn get_provider_adapter(provider: &LLMProvider) -> Arc<dyn ProviderAdapter> {
    match provider.provider_type.as_str() {
        "gpt" => Arc::new(OpenAIAdapter),
        "claude" => Arc::new(AnthropicAdapter),
        // Add more providers as they are implemented
        _ => panic!("Unsupported provider type for function calling: {}", provider.provider_type),
    }
}
