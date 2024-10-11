use crate::schema::user_llm_configs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = user_llm_configs)]
pub struct UserLLMConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub api_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_llm_configs)]
pub struct NewUserLLMConfig {
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub api_key: String,
}
