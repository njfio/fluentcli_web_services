use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::{Attachment, NewAttachment};
use crate::models::conversation::{Conversation, NewConversation};
use crate::models::llm_provider::{LLMProvider, NewLLMProvider};
use crate::models::message::{Message, NewMessage};
use crate::models::user_llm_config::{NewUserLLMConfig, UserLLMConfig};
use crate::schema::{attachments, conversations, llm_providers, messages, user_llm_configs};
use diesel::prelude::*;
use log::{error, info};
use serde_json::Value;
use uuid::Uuid;

pub struct ChatService;

impl ChatService {
    // ... (keep all other functions unchanged)

    pub fn create_message(
        pool: &DbPool,
        _conversation_id: Uuid,
        _role: String,
        _content: Value,
    ) -> Result<Message, AppError> {
        use crate::schema::messages::dsl::*;

        info!(
            "Creating new message - conversation_id: {:?}, role: {:?}, content: {:?}",
            _conversation_id, _role, _content
        );

        let content_string = match _content {
            Value::String(s) => s,
            _ => _content.to_string(),
        };

        info!("Content to be stored: {:?}", content_string);

        let new_message = NewMessage {
            conversation_id: _conversation_id,
            role: _role,
            content: content_string,
        };

        let mut conn = pool.get()?;

        conn.transaction(|conn| {
            info!("Starting transaction to insert new message");
            let result = diesel::insert_into(messages)
                .values(&new_message)
                .get_result::<Message>(conn);

            match result {
                Ok(message) => {
                    info!(
                        "Successfully created message within transaction: {:?}",
                        message
                    );
                    Ok(message)
                }
                Err(e) => {
                    error!("Error creating message within transaction: {:?}", e);
                    Err(AppError::DatabaseError(e))
                }
            }
        })
    }

    // ... (keep all other functions unchanged)
}
