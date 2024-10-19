use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub provider_model: String,
    pub attachment_id: Option<Uuid>,
    pub raw_output: Option<String>,
    pub usage_stats: Option<Value>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub provider_model: String,
    pub attachment_id: Option<Uuid>,
    pub raw_output: Option<String>,
    pub usage_stats: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent {
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
}

impl Message {
    pub fn parse_content(&self) -> Result<MessageContent, serde_json::Error> {
        serde_json::from_str(&self.content)
    }
}

impl NewMessage {
    pub fn new(
        conversation_id: Uuid,
        role: String,
        content: Value,
        provider_model: String,
        attachment_id: Option<Uuid>,
        raw_output: Option<String>,
        usage_stats: Option<Value>,
    ) -> Self {
        NewMessage {
            conversation_id,
            role,
            content: content.to_string(),
            provider_model,
            attachment_id,
            raw_output,
            usage_stats,
        }
    }
}
