use crate::schema::llm_providers;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = llm_providers)]
pub struct LLMProvider {
    pub id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Value,
    pub configuration: Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = llm_providers)]
pub struct NewLLMProvider {
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: String,
    pub supported_modalities: Value,
    pub configuration: Value,
}
