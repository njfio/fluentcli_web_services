use crate::schema::agent_sessions;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = agent_sessions)]
pub struct AgentSession {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = agent_sessions)]
pub struct NewAgentSession {
    pub conversation_id: Uuid,
}
