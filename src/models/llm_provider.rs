use crate::schema::llm_providers;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = llm_providers)]
pub struct LLMProvider {
    pub id: Uuid,
    pub name: String,
    pub api_endpoint: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = llm_providers)]
pub struct NewLLMProvider {
    pub name: String,
    pub api_endpoint: String,
}
