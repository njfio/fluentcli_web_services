use crate::schema::docker_files;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = docker_files)]
pub struct DockerFile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}