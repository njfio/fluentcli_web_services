use crate::schema::amber_store;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = amber_store)]
pub struct AmberStore {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[table_name = "amber_store"]
pub struct NewAmberStore {
    pub user_id: Uuid,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct NewAmberStorePayload {
    pub data: serde_json::Value,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "amber_store"]
pub struct UpdateAmberStore {
    pub data: Option<serde_json::Value>,
}