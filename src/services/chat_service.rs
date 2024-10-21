use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::{Attachment, AttachmentType};
use crate::models::conversation::Conversation;
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use crate::models::message::Message;
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::services::attachment_service::AttachmentService;
use crate::services::chat::{ConversationService, MessageService, UserLLMConfigService};
use crate::services::llm_provider::LLMProviderService;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug)]
pub struct ChatService;

impl ChatService {
    // Conversation-related methods
    pub fn create_conversation(
        pool: &DbPool,
        user_id: Uuid,
        title: String,
    ) -> Result<Conversation, AppError> {
        ConversationService::create_conversation(pool, user_id, title)
    }

    pub fn get_conversation(
        pool: &DbPool,
        conversation_id: Uuid,
    ) -> Result<Conversation, AppError> {
        ConversationService::get_conversation(pool, conversation_id)
    }

    pub fn list_conversations(pool: &DbPool, user_id: Uuid) -> Result<Vec<Conversation>, AppError> {
        ConversationService::list_conversations(pool, user_id)
    }

    pub fn delete_conversation(pool: &DbPool, conversation_id: Uuid) -> Result<(), AppError> {
        ConversationService::delete_conversation(pool, conversation_id)
    }

    // Message-related methods
    pub async fn create_message_with_attachments(
        pool: &DbPool,
        conversation_id: Uuid,
        role: String,
        content: String,
        provider_model: String,
        raw_output: Option<String>,
        usage_stats: Option<Value>,
    ) -> Result<Message, AppError> {
        MessageService::create_message_with_attachments(
            pool,
            conversation_id,
            role,
            content,
            provider_model,
            raw_output,
            usage_stats,
        )
        .await
    }

    pub fn create_message(
        pool: &DbPool,
        conversation_id: Uuid,
        role: String,
        content: Value,
        provider_model: String,
        attachment_id: Option<Uuid>,
        raw_output: Option<String>,
        usage_stats: Option<Value>,
    ) -> Result<Message, AppError> {
        MessageService::create_message(
            pool,
            conversation_id,
            role,
            content,
            provider_model,
            attachment_id,
            raw_output,
            usage_stats,
        )
    }

    pub fn get_message(pool: &DbPool, message_id: Uuid) -> Result<Message, AppError> {
        MessageService::get_message(pool, message_id)
    }

    pub fn get_messages(pool: &DbPool, conversation_id: Uuid) -> Result<Vec<Message>, AppError> {
        MessageService::get_messages(pool, conversation_id)
    }

    // Attachment-related methods
    pub fn create_attachment(
        pool: &DbPool,
        message_id: Uuid,
        file_type: AttachmentType,
        file_path: String,
    ) -> Result<Attachment, AppError> {
        AttachmentService::create_attachment(pool, message_id, file_type, file_path)
    }

    pub fn get_attachments(pool: &DbPool, message_id: Uuid) -> Result<Vec<Attachment>, AppError> {
        AttachmentService::get_attachments(pool, message_id)
    }

    // LLM Provider-related methods
    pub fn get_llm_providers(pool: &DbPool) -> Result<Vec<LLMProvider>, AppError> {
        LLMProviderService::get_llm_providers(pool)
    }

    pub fn create_llm_provider(
        pool: &DbPool,
        new_provider: NewLLMProvider,
    ) -> Result<LLMProvider, AppError> {
        LLMProviderService::create_llm_provider(pool, new_provider)
    }

    pub fn get_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<LLMProvider, AppError> {
        LLMProviderService::get_llm_provider(pool, provider_id)
    }

    pub fn update_llm_provider(
        pool: &DbPool,
        provider_id: Uuid,
        updated_provider: NewLLMProvider,
    ) -> Result<LLMProvider, AppError> {
        LLMProviderService::update_llm_provider(pool, provider_id, updated_provider)
    }

    pub fn delete_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<usize, AppError> {
        LLMProviderService::delete_llm_provider(pool, provider_id)
    }

    // User LLM Config-related methods
    pub fn create_user_llm_config(
        pool: &DbPool,
        user_id: Uuid,
        provider_id: Uuid,
        api_key_id: Uuid,
        description: Option<String>,
    ) -> Result<UserLLMConfig, AppError> {
        UserLLMConfigService::create_user_llm_config(
            pool,
            user_id,
            provider_id,
            api_key_id,
            description,
        )
    }

    pub fn get_user_llm_config(
        pool: &DbPool,
        user_id: Uuid,
        provider_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        UserLLMConfigService::get_user_llm_config(pool, user_id, provider_id)
    }

    pub fn get_user_llm_config_by_id(
        pool: &DbPool,
        config_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        UserLLMConfigService::get_user_llm_config_by_id(pool, config_id)
    }

    pub fn list_user_llm_configs(
        pool: &DbPool,
        user_id: Uuid,
    ) -> Result<Vec<UserLLMConfig>, AppError> {
        UserLLMConfigService::list_user_llm_configs(pool, user_id)
    }

    pub fn update_user_llm_config(
        pool: &DbPool,
        config_id: Uuid,
        updated_config: NewUserLLMConfig,
    ) -> Result<UserLLMConfig, AppError> {
        UserLLMConfigService::update_user_llm_config(pool, config_id, updated_config)
    }

    pub fn delete_user_llm_config(pool: &DbPool, config_id: Uuid) -> Result<usize, AppError> {
        UserLLMConfigService::delete_user_llm_config(pool, config_id)
    }
}
