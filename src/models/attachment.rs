use crate::models::message::Message;
use crate::schema::attachments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttachmentType {
    Text,
    Document,
    Video,
    Image,
    Audio,
}

impl ToString for AttachmentType {
    fn to_string(&self) -> String {
        match self {
            AttachmentType::Text => "Text".to_string(),
            AttachmentType::Document => "Document".to_string(),
            AttachmentType::Video => "Video".to_string(),
            AttachmentType::Image => "Image".to_string(),
            AttachmentType::Audio => "Audio".to_string(),
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Message))]
#[diesel(table_name = attachments)]
pub struct Attachment {
    pub id: Uuid,
    pub message_id: Uuid,
    pub file_type: String,
    pub file_path: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = attachments)]
pub struct NewAttachment {
    pub message_id: Uuid,
    pub file_type: String,
    pub file_path: String,
}

impl Attachment {
    pub fn get_attachment_type(&self) -> AttachmentType {
        match self.file_type.as_str() {
            "Text" => AttachmentType::Text,
            "Document" => AttachmentType::Document,
            "Video" => AttachmentType::Video,
            "Image" => AttachmentType::Image,
            "Audio" => AttachmentType::Audio,
            _ => AttachmentType::Text, // Default to Text if unknown
        }
    }
}

impl NewAttachment {
    pub fn new(message_id: Uuid, attachment_type: AttachmentType, file_path: String) -> Self {
        NewAttachment {
            message_id,
            file_type: attachment_type.to_string(),
            file_path,
        }
    }
}
