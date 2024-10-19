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
    pub fn create_conversation(
        pool: &DbPool,
        _user_id: Uuid,
        _title: String,
    ) -> Result<Conversation, AppError> {
        use crate::schema::conversations::dsl::*;

        info!(
            "Creating new conversation - user_id: {:?}, title: {:?}",
            _user_id, _title
        );

        let new_conversation = NewConversation {
            user_id: _user_id,
            title: _title,
        };

        diesel::insert_into(conversations)
            .values(&new_conversation)
            .get_result::<Conversation>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error creating conversation: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_conversation(
        pool: &DbPool,
        _conversation_id: Uuid,
    ) -> Result<Conversation, AppError> {
        use crate::schema::conversations::dsl::*;

        info!("Fetching conversation with id: {:?}", _conversation_id);

        conversations
            .find(_conversation_id)
            .first::<Conversation>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching conversation: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn list_conversations(
        pool: &DbPool,
        _user_id: Uuid,
    ) -> Result<Vec<Conversation>, AppError> {
        use crate::schema::conversations::dsl::*;

        info!("Listing conversations for user_id: {:?}", _user_id);

        conversations
            .filter(user_id.eq(_user_id))
            .load::<Conversation>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error listing conversations: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn create_message(
        pool: &DbPool,
        _conversation_id: Uuid,
        _role: String,
        _content: Value,
        _provider_model: String,
        _attachment_id: Option<Uuid>,
        _raw_output: Option<String>,
        _usage_stats: Option<Value>,
    ) -> Result<Message, AppError> {
        use crate::schema::messages::dsl::*;

        info!(
            "Creating new message - conversation_id: {:?}, role: {:?}, content: {:?}, provider_model: {:?}",
            _conversation_id, _role, _content, _provider_model
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
            provider_model: _provider_model,
            attachment_id: _attachment_id,
            raw_output: _raw_output,
            usage_stats: _usage_stats,
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

    pub fn get_message(pool: &DbPool, _message_id: Uuid) -> Result<Message, AppError> {
        use crate::schema::messages::dsl::*;

        info!("Fetching message with id: {:?}", _message_id);

        messages
            .find(_message_id)
            .first::<Message>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching message: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_messages(pool: &DbPool, _conversation_id: Uuid) -> Result<Vec<Message>, AppError> {
        use crate::schema::messages::dsl::*;

        info!(
            "Fetching messages for conversation_id: {:?}",
            _conversation_id
        );

        let result = messages
            .filter(conversation_id.eq(_conversation_id))
            .load::<Message>(&mut pool.get()?);

        match result {
            Ok(msgs) => {
                info!("Successfully fetched {} messages", msgs.len());
                for (i, msg) in msgs.iter().enumerate() {
                    info!("Message {}: {:?}", i, msg);
                }
                Ok(msgs)
            }
            Err(e) => {
                error!("Error fetching messages: {:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn create_attachment(
        pool: &DbPool,
        _message_id: Uuid,
        _file_type: String,
        _file_path: String,
    ) -> Result<Attachment, AppError> {
        use crate::schema::attachments::dsl::*;

        info!(
            "Creating new attachment - message_id: {:?}, file_type: {:?}",
            _message_id, _file_type
        );

        let new_attachment = NewAttachment {
            message_id: _message_id,
            file_type: _file_type,
            file_path: _file_path,
        };

        diesel::insert_into(attachments)
            .values(&new_attachment)
            .get_result::<Attachment>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error creating attachment: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_attachments(pool: &DbPool, _message_id: Uuid) -> Result<Vec<Attachment>, AppError> {
        use crate::schema::attachments::dsl::*;

        info!("Fetching attachments for message_id: {:?}", _message_id);

        attachments
            .filter(message_id.eq(_message_id))
            .load::<Attachment>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching attachments: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_llm_providers(pool: &DbPool) -> Result<Vec<LLMProvider>, AppError> {
        use crate::schema::llm_providers::dsl::*;
        let mut conn = pool.get()?;
        let providers = llm_providers.load::<LLMProvider>(&mut conn)?;
        Ok(providers)
    }

    pub fn create_llm_provider(
        pool: &DbPool,
        _user_id: Uuid,
        _name: String,
        _provider_type: String,
        _api_endpoint: String,
        _supported_modalities: Value,
        _configuration: Value,
    ) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        info!(
            "Creating new LLM provider - name: {:?}, type: {:?}, api_endpoint: {:?}",
            _name, _provider_type, _api_endpoint
        );

        let new_provider = NewLLMProvider {
            user_id: _user_id,
            name: _name,
            provider_type: _provider_type,
            api_endpoint: _api_endpoint,
            supported_modalities: _supported_modalities,
            configuration: _configuration,
        };

        diesel::insert_into(llm_providers)
            .values(&new_provider)
            .get_result::<LLMProvider>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error creating LLM provider: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_llm_provider(pool: &DbPool, _provider_id: Uuid) -> Result<LLMProvider, AppError> {
        use crate::schema::llm_providers::dsl::*;

        info!("Fetching LLM provider with id: {:?}", _provider_id);

        llm_providers
            .find(_provider_id)
            .first::<LLMProvider>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching LLM provider: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn create_user_llm_config(
        pool: &DbPool,
        _user_id: Uuid,
        _provider_id: Uuid,
        _api_key_id: Uuid,
        _description: Option<String>,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!(
            "Creating new user LLM config - user_id: {:?}, provider_id: {:?}, api_key_id: {:?}, description: {:?}",
            _user_id, _provider_id, _api_key_id, _description
        );

        let new_config = NewUserLLMConfig {
            user_id: _user_id,
            provider_id: _provider_id,
            api_key_id: _api_key_id,
            description: _description,
        };

        let result = diesel::insert_into(user_llm_configs)
            .values(&new_config)
            .get_result::<UserLLMConfig>(&mut pool.get()?);

        match result {
            Ok(config) => {
                info!("Successfully created user LLM config: {:?}", config);
                Ok(config)
            }
            Err(e) => {
                error!("Error creating user LLM config: {:?}", e);
                error!("Attempted to insert: {:?}", new_config);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn get_user_llm_config(
        pool: &DbPool,
        _user_id: Uuid,
        _provider_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!(
            "Fetching user LLM config - user_id: {:?}, provider_id: {:?}",
            _user_id, _provider_id
        );

        user_llm_configs
            .filter(user_id.eq(_user_id))
            .filter(provider_id.eq(_provider_id))
            .first::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching user LLM config: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn get_user_llm_config_by_id(
        pool: &DbPool,
        _config_id: Uuid,
    ) -> Result<UserLLMConfig, AppError> {
        use crate::schema::user_llm_configs::dsl::*;

        info!("Fetching user LLM config by id: {:?}", _config_id);

        user_llm_configs
            .find(_config_id)
            .first::<UserLLMConfig>(&mut pool.get()?)
            .map_err(|e| {
                error!("Error fetching user LLM config by id: {:?}", e);
                AppError::DatabaseError(e)
            })
    }

    pub fn delete_conversation(pool: &DbPool, _conversation_id: Uuid) -> Result<(), AppError> {
        use crate::schema::conversations::dsl::*;
        use crate::schema::messages::dsl::*;

        info!("Deleting conversation with id: {:?}", _conversation_id);

        let mut conn = pool.get()?;

        conn.transaction(|conn| {
            // First, delete all messages associated with the conversation
            diesel::delete(messages.filter(conversation_id.eq(_conversation_id)))
                .execute(conn)
                .map_err(|e| {
                    error!("Error deleting messages: {:?}", e);
                    AppError::DatabaseError(e)
                })?;

            // Then, delete the conversation itself
            diesel::delete(conversations.find(_conversation_id))
                .execute(conn)
                .map_err(|e| {
                    error!("Error deleting conversation: {:?}", e);
                    AppError::DatabaseError(e)
                })?;

            Ok(())
        })
    }
}
