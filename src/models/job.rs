use crate::schema::jobs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = jobs)]
pub struct Job {
    pub id: Uuid,
    pub user_id: Uuid,
    pub uri: String,
    pub config: Value, // Reference to configuration ID
    pub amber_id: Option<Uuid>, // Reference to amber_store ID
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String, // Reference to docker_file ID
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[table_name = "jobs"]
pub struct NewJob {
    pub user_id: Uuid,
    pub uri: String,
    pub config: Value,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct NewJobPayload {
    pub uri: String,
    pub config: Value,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "jobs"]
pub struct UpdateJob {
    pub uri: Option<String>,
    pub config: Option<Value>,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: Option<String>,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: Option<String>,
}