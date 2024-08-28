use crate::schema::active_workers;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = active_workers)]
pub struct ActiveWorker {
    pub id: Uuid,
    pub user_id: Uuid,
    pub worker_type: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}