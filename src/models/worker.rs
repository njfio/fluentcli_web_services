use crate::schema::workers;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = workers)]
pub struct Worker {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub worker_type: Uuid, // Reference to docker_file ID
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = workers)]
pub struct NewWorker {
    pub user_id: Uuid,
    pub name: String,
    pub worker_type: Uuid,
    pub active: bool,
}

#[derive(Deserialize, Debug)]
pub struct NewWorkerPayload {
    pub name: String,
    pub worker_type: Uuid,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = workers)]
pub struct UpdateWorker {
    pub name: Option<String>,
    pub worker_type: Option<Uuid>,
    pub active: Option<bool>,
}