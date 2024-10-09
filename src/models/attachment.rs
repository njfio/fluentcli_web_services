use crate::schema::attachments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
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
