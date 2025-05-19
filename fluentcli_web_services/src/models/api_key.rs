use crate::schema::api_keys;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    #[diesel(column_name = "key_value")]
    pub key_value: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub user_id: Uuid,
    pub key_value: String,
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
