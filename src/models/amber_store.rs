use crate::schema::amber_store;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = amber_store)]
pub struct AmberStore {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Insertable, Debug)]
#[table_name = "amber_store"]
pub struct NewAmberStore {
    pub user_id: Uuid,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct NewAmberStorePayload {
    pub data: serde_yaml::Value,
}

#[derive(AsChangeset, Deserialize, Debug )]
#[table_name = "amber_store"]
pub struct UpdateAmberStore {
    pub data: Option<String>,
}