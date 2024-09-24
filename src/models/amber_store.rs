use crate::schema::amber_store;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = amber_store)]
pub struct AmberStore {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub data: String,
    #[serde(skip_serializing)]
    pub secure_key_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[table_name = "amber_store"]
pub struct NewAmberStore {
    pub user_id: Uuid,
    pub name: String,
    pub data: String,
    pub secure_key_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct NewAmberStorePayload {
    pub name: String,
    pub data: serde_yaml::Value,
    pub secure_key_hash: String,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "amber_store"]
pub struct UpdateAmberStore {
    pub name: Option<String>,
    pub data: Option<String>,
    pub secure_key_hash: Option<String>,
}