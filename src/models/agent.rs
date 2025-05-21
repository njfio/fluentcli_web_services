use crate::schema::agents;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = agents)]
pub struct Agent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub tools: Value, // JSON array of tool IDs
    pub icon: Option<String>,
    pub system_prompt: Option<String>,
    pub reasoning_patterns: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = agents)]
pub struct NewAgent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub tools: Value, // JSON array of tool IDs
    pub icon: Option<String>,
    pub system_prompt: Option<String>,
    pub reasoning_patterns: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateAgentRequest {
    pub name: String,
    pub description: String,
    pub tools: Vec<String>, // Array of tool IDs
    pub icon: Option<String>,
    pub system_prompt: Option<String>,
    pub reasoning_patterns: Option<Vec<String>>,
}

// Convert CreateAgentRequest to NewAgent for database insertion
impl From<CreateAgentRequest> for NewAgent {
    fn from(req: CreateAgentRequest) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::nil(), // This will be set by the service
            name: req.name,
            description: req.description,
            tools: json!(req.tools),
            icon: req.icon,
            system_prompt: req.system_prompt,
            reasoning_patterns: req.reasoning_patterns.map(|v| json!(v)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = agents)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_tools")]
    pub tools: Option<Value>, // JSON array of tool IDs
    pub icon: Option<String>,
    pub system_prompt: Option<String>,
    #[serde(deserialize_with = "deserialize_tools")]
    pub reasoning_patterns: Option<Value>,
}

// Custom deserializer for tools field
fn deserialize_tools<'de, D>(deserializer: D) -> Result<Option<Value>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // First try to deserialize as a Vec<String>
    let tools_vec: Option<Vec<String>> = Option::deserialize(deserializer)?;

    // Convert Vec<String> to Value if present
    Ok(tools_vec.map(|v| json!(v)))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tools: Vec<String>, // Array of tool IDs
    pub icon: Option<String>,
    pub system_prompt: Option<String>,
    pub reasoning_patterns: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Agent> for AgentResponse {
    fn from(agent: Agent) -> Self {
        let tools = match agent.tools {
            Value::Array(arr) => arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            _ => Vec::new(),
        };

        Self {
            id: agent.id,
            name: agent.name,
            description: agent.description,
            tools,
            icon: agent.icon,
            system_prompt: agent.system_prompt,
            reasoning_patterns: agent.reasoning_patterns.and_then(|v| {
                v.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(|s| s.to_string()))
                        .collect()
                })
            }),
            created_at: agent.created_at,
            updated_at: agent.updated_at,
        }
    }
}
