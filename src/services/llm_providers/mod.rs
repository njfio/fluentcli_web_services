pub mod anthropic;
pub mod cohere;
pub mod dalle;
pub mod gemini;
pub mod openai;
pub mod perplexity;

pub use anthropic::AnthropicProvider;
pub use cohere::CohereProvider;
pub use dalle::DalleProvider;
pub use gemini::GeminiProvider;
pub use openai::OpenAIProvider;
pub use perplexity::PerplexityProvider;

use crate::services::llm_service::{LLMChatMessage, LLMProviderTrait, LLMServiceError};
use serde_json::Value;

pub fn get_provider(provider_type: &str) -> Box<dyn LLMProviderTrait> {
    match provider_type {
        "claude" => Box::new(AnthropicProvider),
        "command" => Box::new(CohereProvider),
        "dalle" => Box::new(DalleProvider),
        "gemini" => Box::new(GeminiProvider),
        "gpt" => Box::new(OpenAIProvider),
        "perplexity" => Box::new(PerplexityProvider),
        _ => panic!("Unknown provider type: {}", provider_type),
    }
}
