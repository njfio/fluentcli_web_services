use crate::db::DbPool;
use crate::error::AppError;
use crate::models::message::{Message, NewMessage};
use crate::schema::messages;
use crate::services::attachment_service::AttachmentService;
use diesel::prelude::*;
use log::{error, info};
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs;
use tokio::sync::oneshot;
use uuid::Uuid;

pub struct MessageService;

impl MessageService {
    async fn process_stability_image(
        pool: &DbPool,
        message_id: Uuid,
        image_data: &str,
    ) -> Result<String, AppError> {
        // Decode base64 image data
        let image_bytes = base64::decode(image_data).map_err(|e| {
            error!("Failed to decode base64 image data: {:?}", e);
            AppError::InternalServerError
        })?;

        // Create attachments directory if it doesn't exist
        let attachment_dir = AttachmentService::get_attachment_dir();
        fs::create_dir_all(&attachment_dir).await.map_err(|e| {
            error!("Failed to create attachments directory: {:?}", e);
            AppError::InternalServerError
        })?;

        // Generate a unique filename
        let file_name = format!("{}.png", Uuid::new_v4());
        let file_path = attachment_dir.join(&file_name);

        // Save the image
        fs::write(&file_path, &image_bytes).await.map_err(|e| {
            error!("Failed to write image file: {:?}", e);
            AppError::InternalServerError
        })?;

        // Create attachment record
        let attachment = AttachmentService::create_attachment(
            pool,
            message_id,
            crate::models::attachment::AttachmentType::Image,
            file_path.to_str().unwrap().to_string(),
        )?;

        Ok(format!("{{{{ATTACHMENT:{}}}}}", attachment.id))
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

                // Try to insert the message, handling potential schema issues
                let insert_result = diesel::insert_into(messages::table)
                    .values(&new_message)
                    .get_result(conn);

                match insert_result {
                    Ok(message) => {
                        info!("Message created: {:?}", message);
                        Ok(message)
                    }
                    Err(e) => {
                        // Check if the error is related to the attachment_id column
                        if e.to_string().contains("attachment_id") {
                            error!("Error with attachment_id column: {:?}", e);
                            // Try inserting without the attachment_id field
                            let simplified_message = (
                                messages::conversation_id.eq(conversation_id),
                                messages::role.eq(role.clone()),
                                messages::content.eq(content.clone()),
                                messages::provider_model.eq(provider_model.clone()),
                                messages::raw_output.eq(raw_output.clone()),
                                messages::usage_stats.eq(usage_stats.clone()),
                            );

                            diesel::insert_into(messages::table)
                                .values(simplified_message)
                                .get_result(conn)
                        } else {
                            Err(e)
                        }
                    }
                }
            })
            .map_err(AppError::DatabaseError)?;

        // Process attachments asynchronously and wait for the result
        let pool_clone = pool.clone();
        let content_clone = content.clone();
        let message_id = message.id;
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = if content_clone.starts_with("STABILITY_IMAGE_DATA:") {
                // Handle Stability AI image data
                match Self::process_stability_image(
                    &pool_clone,
                    message_id,
                    &content_clone[19..], // Skip the "STABILITY_IMAGE_DATA:" prefix
                )
                .await
                {
                    Ok(attachment_content) => {
                        let conn = &mut pool_clone.get().expect("Failed to get DB connection");
                        let update_result = diesel::update(messages::table.find(message_id))
                            .set(messages::content.eq(&attachment_content))
                            .execute(conn);

                        match update_result {
                            Ok(_) => {
                                info!("Updated message with Stability AI attachment");
                                messages::table.find(message_id).first::<Message>(conn).ok()
                            }
                            Err(e) => {
                                error!("Failed to update message with attachment: {:?}", e);
                                None
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to process Stability AI image: {:?}", e);
                        None
                    }
                }
            } else {
                // Handle regular image URLs
                match AttachmentService::process_and_store_images(
                    &pool_clone,
                    message_id,
                    &content_clone,
                )
                .await
                {
                    Ok(attachments) => {
                        info!("Processed {} attachments", attachments.len());
                        if let Some(main_attachment) = attachments.first() {
                            let updated_content =
                                AttachmentService::replace_urls_with_attachment_ids(
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
                                    messages::table.find(message_id).first::<Message>(conn).ok()
                                }
                                Err(e) => {
                                    error!("Failed to update message with attachment: {:?}", e);
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        error!("Failed to process attachments: {:?}", e);
                        None
                    }
                }
            };

            let _ = tx.send(result);
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

        // Create a new message with or without attachment_id based on schema
        let new_message = match _attachment_id {
            Some(attachment_id) => {
                // Try to create with attachment_id
                info!("Creating message with attachment_id: {}", attachment_id);
                NewMessage {
                    conversation_id: _conversation_id,
                    role: _role,
                    content: content_string,
                    provider_model: _provider_model,
                    attachment_id: Some(attachment_id),
                    raw_output: _raw_output,
                    usage_stats: _usage_stats,
                }
            }
            None => {
                // Create without attachment_id
                info!("Creating message without attachment_id");
                NewMessage {
                    conversation_id: _conversation_id,
                    role: _role,
                    content: content_string,
                    provider_model: _provider_model,
                    attachment_id: None,
                    raw_output: _raw_output,
                    usage_stats: _usage_stats,
                }
            }
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
}
