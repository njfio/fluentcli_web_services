use crate::schema::pipelines;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = pipelines)]
pub struct Pipeline {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[table_name = "pipelines"]
pub struct NewPipeline {
    pub user_id: Uuid,
    pub name: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct NewPipelinePayload {
    pub name: String,
    pub data: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "pipelines"]
pub struct UpdatePipeline {
    pub name: Option<String>,
    pub data: Option<String>,
}
