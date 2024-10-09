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
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub user_id: uuid::Uuid,
    pub title: String,
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
