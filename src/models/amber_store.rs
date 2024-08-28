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
    pub data: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}