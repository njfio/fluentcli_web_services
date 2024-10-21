use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::attachment::Attachment;
use crate::models::message::{Message, NewMessage};
use crate::schema::messages;
use crate::services::attachment_service::AttachmentService;
use diesel::prelude::*;
use log::{debug, error, info};
use uuid::Uuid;

pub struct ChatService;

impl ChatService {
    pub async fn create_message_with_attachments(
        pool: &DbPool,
        conversation_id: Uuid,
        role: String,
        content: String,
        provider_model: String,
        raw_output: Option<String>,
        usage_stats: Option<serde_json::Value>,
    ) -> Result<Message, AppError> {
        info!(
            "Creating message with attachments for conversation: {}",
            conversation_id
        );

        let conn = &mut pool.get()?;
        conn.transaction(|conn| {
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

            // Process and store images
            match tokio::task::block_in_place(|| {
                tokio::runtime::Runtime::new().unwrap().block_on(
                    AttachmentService::process_and_store_images(pool, message.id, &content),
                )
            }) {
                Ok(attachments) => {
                    info!("Processed {} attachments", attachments.len());
                    if let Some(main_attachment) = attachments.first() {
                        // Update the message with the first attachment ID and replace URLs with placeholders
                        let updated_content = AttachmentService::replace_urls_with_attachment_ids(
                            &content,
                            &attachments,
                        );
                        let updated_message = diesel::update(&message)
                            .set((
                                messages::attachment_id.eq(main_attachment.id),
                                messages::content.eq(updated_content),
                            ))
                            .get_result::<Message>(conn)?;
                        info!("Updated message with attachment information");
                        Ok(updated_message)
                    } else {
                        Ok(message)
                    }
                }
                Err(e) => {
                    error!("Failed to process attachments: {:?}", e);
                    Err(diesel::result::Error::RollbackTransaction)
                }
            }
        })
        .map_err(AppError::DatabaseError)
    }

    // ... (keep other methods unchanged)
}
