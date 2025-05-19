use crate::schema::configurations;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = configurations)]
pub struct Configuration {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub data: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Insertable, Debug)]
#[diesel(table_name = configurations)]
pub struct NewConfiguration {
    pub user_id: Uuid,
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct NewConfigurationPayload {
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "configurations"]
pub struct UpdateConfiguration {
    pub name: Option<String>,
    pub data: Option<serde_json::Value>,
}