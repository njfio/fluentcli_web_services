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
    pub uri: Uuid,
    pub config: Value,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
    pub results: Option<Value>,
    pub pipeline_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[table_name = "jobs"]
pub struct NewJob {
    pub user_id: Uuid,
    pub uri: Uuid,
    pub config: Value,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
    pub pipeline_id: Uuid,
    pub results: Option<Value>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "jobs"]
pub struct UpdateJob {
    pub config: Option<Value>,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: Option<String>,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: Option<String>,
    pub results: Option<Value>,
    pub pipeline_id: Option<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct NewJobPayload {
    pub uri: Option<String>,
    pub config: Value,
    pub amber_id: Option<Uuid>,
    pub state_file_content: Option<String>,
    pub data_path: Option<String>,
    pub worker_type: String,
    pub triggers: Option<Value>,
    pub timers: Option<Value>,
    pub status: String,
    pub pipeline_id: Uuid,
    pub results: Option<Value>,
}