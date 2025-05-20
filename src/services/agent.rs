use crate::db::DbPool;
use crate::error::AppError;
use crate::models::agent::{Agent, AgentResponse, CreateAgentRequest, NewAgent, UpdateAgentRequest};
use crate::schema::agents;
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;
use log::{debug, error, info};

pub struct AgentService;

impl AgentService {
    pub fn create_agent(
        pool: &DbPool,
        user_id: Uuid,
        request: CreateAgentRequest,
    ) -> Result<AgentResponse, AppError> {
        let conn = &mut pool.get()?;

        debug!("Creating agent for user {}: {}", user_id, request.name);

        // Validate the tools exist
        // In a real implementation, you would check if the tools exist in the tool registry

        let tools_json = json!(request.tools);

        let new_agent = NewAgent {
            id: Uuid::new_v4(),
            user_id,
            name: request.name,
            description: request.description,
            tools: tools_json,
            icon: request.icon,
            system_prompt: request.system_prompt,
        };

        let agent = diesel::insert_into(agents::table)
            .values(&new_agent)
            .get_result::<Agent>(conn)?;

        info!("Created agent with ID: {}", agent.id);
        Ok(AgentResponse::from(agent))
    }

    pub fn get_agent(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<AgentResponse, AppError> {
        let conn = &mut pool.get()?;

        debug!("Getting agent {} for user {}", id, user_id);

        let agent = agents::table
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .first::<Agent>(conn)
            .map_err(|e| {
                if let diesel::result::Error::NotFound = e {
                    AppError::NotFound(format!("Agent not found: {}", id))
                } else {
                    AppError::DatabaseError(e)
                }
            })?;

        Ok(AgentResponse::from(agent))
    }

    pub fn list_agents(pool: &DbPool, user_id: Uuid) -> Result<Vec<AgentResponse>, AppError> {
        let conn = &mut pool.get()?;

        debug!("Listing agents for user {}", user_id);

        let agents = agents::table
            .filter(agents::user_id.eq(user_id))
            .load::<Agent>(conn)?;

        debug!("Found {} agents for user {}", agents.len(), user_id);
        Ok(agents.into_iter().map(AgentResponse::from).collect())
    }

    pub fn update_agent(
        pool: &DbPool,
        id: Uuid,
        user_id: Uuid,
        request: UpdateAgentRequest,
    ) -> Result<AgentResponse, AppError> {
        let conn = &mut pool.get()?;

        debug!("Updating agent {} for user {}", id, user_id);

        // First, check if the agent exists and belongs to the user
        let agent = agents::table
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .first::<Agent>(conn)
            .map_err(|e| {
                if let diesel::result::Error::NotFound = e {
                    AppError::NotFound(format!("Agent not found: {}", id))
                } else {
                    AppError::DatabaseError(e)
                }
            })?;

        // Create the update request
        let mut update_request = UpdateAgentRequest {
            name: request.name,
            description: request.description,
            tools: request.tools,
            icon: request.icon,
            system_prompt: request.system_prompt,
        };

        // Update the agent
        let updated_agent = diesel::update(agents::table)
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .set((
                update_request,
                agents::updated_at.eq(Utc::now()),
            ))
            .get_result::<Agent>(conn)?;

        info!("Updated agent {}", id);
        Ok(AgentResponse::from(updated_agent))
    }

    pub fn delete_agent(pool: &DbPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let conn = &mut pool.get()?;

        debug!("Deleting agent {} for user {}", id, user_id);

        let count = diesel::delete(agents::table)
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .execute(conn)?;

        if count == 0 {
            error!("Agent not found for deletion: {}", id);
            return Err(AppError::NotFound(format!("Agent not found: {}", id)));
        }

        info!("Deleted agent {}", id);
        Ok(())
    }
}
