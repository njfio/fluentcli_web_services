use crate::schema::agents;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = agents)]
#[diesel(belongs_to(AgentSession, foreign_key = session_id))]
#[diesel(belongs_to(UserLLMConfig, foreign_key = user_llm_config_id))]
pub struct Agent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_llm_config_id: Uuid,
    pub role_name: String,
    pub tool_config: Option<Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = agents)]
pub struct NewAgent {
    pub session_id: Uuid,
    pub user_llm_config_id: Uuid,
    pub role_name: String,
    pub tool_config: Option<Value>,
}
