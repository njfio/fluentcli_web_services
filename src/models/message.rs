use crate::schema::messages;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
}
