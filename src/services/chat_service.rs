use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::conversation::{Conversation, NewConversation};
use crate::models::llm_provider::LLMProvider;
use crate::models::message::{Message, NewMessage};
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::schema::{conversations, llm_providers, messages, user_llm_configs};
use crate::services::attachment_service::AttachmentService;
use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

pub struct ChatService;

impl ChatService {
    pub fn create_conversation(
        pool: &DbPool,
        user_id: Uuid,
        title: String,
        mode: String,
    ) -> Result<Conversation, AppError> {
        let conn = &mut pool.get()?;

        let new_conversation = NewConversation {
            user_id,
            title,
            mode,
        };

        diesel::insert_into(conversations::table)
            .values(&new_conversation)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_conversation(
        pool: &DbPool,
        conversation_id: Uuid,
    ) -> Result<Conversation, AppError> {
        let conn = &mut pool.get()?;

        conversations::table
            .filter(conversations::id.eq(conversation_id))
            .first(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_conversations(pool: &DbPool, user_id: Uuid) -> Result<Vec<Conversation>, AppError> {
        let conn = &mut pool.get()?;

        conversations::table
            .filter(conversations::user_id.eq(user_id))
            .load::<Conversation>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_conversation(pool: &DbPool, conversation_id: Uuid) -> Result<(), AppError> {
        let conn = &mut pool.get()?;

        diesel::delete(conversations::table.filter(conversations::id.eq(conversation_id)))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;

        Ok(())
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
        let conn = &mut pool.get()?;

        let content_string = match content {
            Value::String(s) => s,
            _ => content.to_string(),
        };

        let new_message = NewMessage {
            conversation_id,
            role,
            content: content_string,
            provider_model,
            attachment_id,
            raw_output,
            usage_stats,
        };

        diesel::insert_into(messages::table)
            .values(&new_message)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub async fn create_message_with_attachments(
        pool: &DbPool,
        conversation_id: Uuid,
        role: String,
        content: String,
        provider_model: String,
        raw_output: Option<String>,
        usage_stats: Option<Value>,
    ) -> Result<Message, AppError> {
        let conn = &mut pool.get()?;

        // First create the message without attachments
        let message = conn.transaction(|conn| {
            let new_message = NewMessage {
                conversation_id,
                role: role.clone(),
                content: content.clone(),
                provider_model: provider_model.clone(),
                attachment_id: None,
                raw_output: raw_output.clone(),
                usage_stats: usage_stats.clone(),
            };

            diesel::insert_into(messages::table)
                .values(&new_message)
                .get_result::<Message>(conn)
                .map_err(AppError::DatabaseError)
        })?;

        // Process attachments if any
        let attachments = AttachmentService::process_and_store_images(pool, message.id, &content)
            .await
            .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;

        if let Some(main_attachment) = attachments.first() {
            // Update message with attachment info
            let updated_content =
                AttachmentService::replace_urls_with_attachment_ids(&content, &attachments);

            diesel::update(messages::table.find(message.id))
                .set((
                    messages::attachment_id.eq(main_attachment.id),
                    messages::content.eq(updated_content),
                ))
                .get_result(conn)
                .map_err(AppError::DatabaseError)
        } else {
            Ok(message)
        }
    }

    pub fn get_message(pool: &DbPool, message_id: Uuid) -> Result<Message, AppError> {
        let conn = &mut pool.get()?;

        messages::table
            .find(message_id)
            .first(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_messages(pool: &DbPool, conversation_id: Uuid) -> Result<Vec<Message>, AppError> {
        let conn = &mut pool.get()?;

        messages::table
            .filter(messages::conversation_id.eq(conversation_id))
            .order(messages::created_at.asc())
            .load::<Message>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_message(
        pool: &DbPool,
        conversation_id: Uuid,
        message_id: Uuid,
    ) -> Result<(), AppError> {
        let conn = &mut pool.get()?;

        diesel::delete(
            messages::table
                .filter(messages::id.eq(message_id))
                .filter(messages::conversation_id.eq(conversation_id)),
        )
        .execute(conn)
        .map_err(AppError::DatabaseError)?;

        Ok(())
    }

    pub fn get_llm_providers(pool: &DbPool) -> Result<Vec<LLMProvider>, AppError> {
        let conn = &mut pool.get()?;

        llm_providers::table
            .load::<LLMProvider>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_llm_provider(pool: &DbPool, provider_id: Uuid) -> Result<LLMProvider, AppError> {
        let conn = &mut pool.get()?;

        llm_providers::table
            .filter(llm_providers::id.eq(provider_id))
            .first::<LLMProvider>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_user_llm_config(
        pool: &DbPool,
        user_id: Uuid,
        provider_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        let conn = &mut pool.get()?;

        user_llm_configs::table
            .filter(user_llm_configs::user_id.eq(user_id))
            .filter(user_llm_configs::provider_id.eq(provider_id))
            .first::<UserLLMConfig>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_user_llm_config_by_id(
        pool: &DbPool,
        config_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        let conn = &mut pool.get()?;

        user_llm_configs::table
            .find(config_id)
            .first::<UserLLMConfig>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn create_user_llm_config(
        pool: &DbPool,
        user_id: Uuid,
        provider_id: Uuid,
        api_key_id: Uuid,
        description: Option<String>,
    ) -> Result<UserLLMConfig, AppError> {
        let conn = &mut pool.get()?;

        let new_config = NewUserLLMConfig {
            user_id,
            provider_id,
            api_key_id,
            description,
        };

        diesel::insert_into(user_llm_configs::table)
            .values(&new_config)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_user_llm_configs(
        pool: &DbPool,
        user_id: Uuid,
    ) -> Result<Vec<UserLLMConfig>, AppError> {
        let conn = &mut pool.get()?;

        user_llm_configs::table
            .filter(user_llm_configs::user_id.eq(user_id))
            .load::<UserLLMConfig>(conn)
            .map_err(AppError::DatabaseError)
    }
}
