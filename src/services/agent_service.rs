use crate::models::agent::{Agent, NewAgent, UpdateAgent};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref AGENTS: Mutex<HashMap<Uuid, Agent>> = Mutex::new(HashMap::new());
}

pub struct AgentService;

impl AgentService {
    pub fn create_agent(new_agent: NewAgent) -> Agent {
        let agent = Agent {
            id: Uuid::new_v4(),
            name: new_agent.name,
            description: new_agent.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let mut map = AGENTS.lock().unwrap();
        map.insert(agent.id, agent.clone());
        agent
    }

    pub fn list_agents() -> Vec<Agent> {
        let map = AGENTS.lock().unwrap();
        map.values().cloned().collect()
    }

    pub fn get_agent(id: Uuid) -> Option<Agent> {
        let map = AGENTS.lock().unwrap();
        map.get(&id).cloned()
    }

    pub fn update_agent(id: Uuid, update: UpdateAgent) -> Option<Agent> {
        let mut map = AGENTS.lock().unwrap();
        if let Some(entry) = map.get_mut(&id) {
            if let Some(name) = update.name {
                entry.name = name;
            }
            if let Some(desc) = update.description {
                entry.description = Some(desc);
            }
            entry.updated_at = Utc::now();
            return Some(entry.clone());
        }
        None
    }

    pub fn delete_agent(id: Uuid) -> bool {
        let mut map = AGENTS.lock().unwrap();
        map.remove(&id).is_some()
    }
}
