use crate::schema::conversations;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = conversations)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub mode: String, // Added mode field to distinguish between 'chat' and 'arena'
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub user_id: uuid::Uuid,
    pub title: String,
    pub mode: String, // Added mode field for new conversations
}

// Add this function to print type information
pub fn print_type_info() {
    println!(
        "Conversation id type: {}",
        std::any::type_name::<uuid::Uuid>()
    );
    println!(
        "Conversation user_id type: {}",
        std::any::type_name::<uuid::Uuid>()
    );
    println!(
        "NewConversation user_id type: {}",
        std::any::type_name::<uuid::Uuid>()
    );
}
