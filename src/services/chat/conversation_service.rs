use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::conversation::{Conversation, NewConversation};
use crate::schema::conversations;
use diesel::prelude::*;
use log::{error, info};
use uuid::Uuid;

pub struct ConversationService;

impl ConversationService {
    pub fn create_conversation(
        pool: &DbPool,
        _user_id: Uuid,
        _title: String,
        _mode: String,
    ) -> Result<Conversation, AppError> {
        use crate::schema::conversations::dsl::*;

        info!(
            "Creating new conversation - user_id: {:?}, title: {:?}, mode: {:?}",
            _user_id, _title, _mode
        );

        let new_conversation = NewConversation {
            user_id: _user_id,
            title: _title,
            mode: _mode,
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
