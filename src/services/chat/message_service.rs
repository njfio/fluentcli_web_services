use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::message::{Message, NewMessage};
use crate::schema::messages;
use crate::services::attachment_service::AttachmentService;
use diesel::prelude::*;
use log::{error, info};
use serde_json::Value;
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct MessageService;

impl MessageService {
    pub async fn create_message_with_attachments(
        pool: &DbPool,
        conversation_id: Uuid,
        role: String,
        content: String,
        provider_model: String,
        raw_output: Option<String>,
        usage_stats: Option<Value>,
    ) -> Result<Message, AppError> {
        info!(
            "Creating message with attachments for conversation: {}",
            conversation_id
        );

        let conn = &mut pool.get()?;
        let message: Message = conn
            .transaction::<_, diesel::result::Error, _>(|conn| {
                // Create the message first
                let new_message = NewMessage {
                    conversation_id,
                    role: role.clone(),
                    content: content.clone(),
                    provider_model: provider_model.clone(),
                    attachment_id: None,
                    raw_output: raw_output.clone(),
                    usage_stats: usage_stats.clone(),
                };

                let message: Message = diesel::insert_into(messages::table)
                    .values(&new_message)
                    .get_result(conn)?;

                info!("Message created: {:?}", message);
                Ok(message)
            })
            .map_err(AppError::DatabaseError)?;

        // Process attachments asynchronously and wait for the result
        let pool_clone = pool.clone();
        let content_clone = content.clone();
        let message_id = message.id;
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = AttachmentService::process_and_store_images(
                &pool_clone,
                message_id,
                &content_clone,
            )
            .await;

            match result {
                Ok(attachments) => {
                    info!("Processed {} attachments", attachments.len());
                    if let Some(main_attachment) = attachments.first() {
                        let updated_content = AttachmentService::replace_urls_with_attachment_ids(
                            &content_clone,
                            &attachments,
                        );
                        let conn = &mut pool_clone.get().expect("Failed to get DB connection");
                        let update_result = diesel::update(messages::table.find(message_id))
                            .set((
                                messages::attachment_id.eq(main_attachment.id),
                                messages::content.eq(updated_content),
                            ))
                            .execute(conn);
                        match update_result {
                            Ok(_) => {
                                info!("Updated message with attachment information");
                                let updated_message =
                                    messages::table.find(message_id).first::<Message>(conn).ok();
                                let _ = tx.send(updated_message);
                            }
                            Err(e) => {
                                error!("Failed to update message with attachment: {:?}", e);
                                let _ = tx.send(None);
                            }
                        }
                    } else {
                        let _ = tx.send(None);
                    }
                }
                Err(e) => {
                    error!("Failed to process attachments: {:?}", e);
                    let _ = tx.send(None);
                }
            }
        });

        // Wait for the attachment processing to complete
        match rx.await {
            Ok(Some(updated_message)) => Ok(updated_message),
            _ => Ok(message),
        }
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
            .order(created_at.asc()) // Order by creation date ascending
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

    pub fn delete_message(
        pool: &DbPool,
        _conversation_id: Uuid,
        _message_id: Uuid,
    ) -> Result<(), AppError> {
        use crate::schema::messages::dsl::*;

        info!(
            "Deleting message with id: {:?} from conversation: {:?}",
            _message_id, _conversation_id
        );

        let conn = &mut pool.get()?;

        // First, check if the message belongs to the specified conversation
        let message_exists = messages
            .filter(id.eq(_message_id))
            .filter(conversation_id.eq(_conversation_id))
            .first::<Message>(conn)
            .optional()
            .map_err(|e| {
                error!("Error checking message existence: {:?}", e);
                AppError::DatabaseError(e)
            })?;

        if message_exists.is_none() {
            error!("Message not found or doesn't belong to the specified conversation");
            return Err(AppError::NotFoundError("Message not found".to_string()));
        }

        // If the message exists and belongs to the conversation, delete it
        diesel::delete(messages.find(_message_id))
            .execute(conn)
            .map_err(|e| {
                error!("Error deleting message: {:?}", e);
                AppError::DatabaseError(e)
            })?;

        info!(
            "Successfully deleted message with id: {:?} from conversation: {:?}",
            _message_id, _conversation_id
        );
        Ok(())
    }

    // ... (keep all other existing methods)
}
