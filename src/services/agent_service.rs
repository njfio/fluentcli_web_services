use crate::db::DbPool;
use crate::error::AppError;
use crate::models::agent::{
    Agent, AgentResponse, CreateAgentRequest, NewAgent, UpdateAgentRequest,
};
use crate::schema::agents;
use chrono::Utc;
use diesel::prelude::*;
use log::{debug, error, info};
use serde_json::json;
use uuid::Uuid;

pub struct AgentService;

impl AgentService {
    pub fn create_agent(
        pool: &DbPool,
        user_id: Uuid,
        request: CreateAgentRequest,
    ) -> Result<AgentResponse, AppError> {
        let conn = &mut pool.get()?;

        debug!("Creating agent for user {}: {}", user_id, request.name);
        info!("Creating agent with tools: {:?}", request.tools);

        // Validate the tools exist
        // In a real implementation, you would check if the tools exist in the tool registry

        // Create a new agent from the request
        let mut new_agent = NewAgent::from(request);

        // Set the user_id
        new_agent.user_id = user_id;

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
                    AppError::NotFoundError(format!("Agent not found: {}", id))
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
        mut request: UpdateAgentRequest,
    ) -> Result<AgentResponse, AppError> {
        let conn = &mut pool.get()?;

        debug!("Updating agent {} for user {}", id, user_id);
        debug!("Update request: {:?}", request);

        // First, check if the agent exists and belongs to the user
        let _agent = agents::table
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .first::<Agent>(conn)
            .map_err(|e| {
                if let diesel::result::Error::NotFound = e {
                    AppError::NotFoundError(format!("Agent not found: {}", id))
                } else {
                    AppError::DatabaseError(e)
                }
            })?;

        // Update the agent
        let updated_agent = diesel::update(agents::table)
            .filter(agents::id.eq(id))
            .filter(agents::user_id.eq(user_id))
            .set(agents::updated_at.eq(Utc::now()))
            .get_result::<Agent>(conn)?;

        // Update individual fields to avoid AsChangeset issues
        if let Some(name) = &request.name {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::name.eq(name))
                .execute(conn)?;
        }

        if let Some(description) = &request.description {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::description.eq(description))
                .execute(conn)?;
        }

        if let Some(tools) = &request.tools {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::tools.eq(tools))
                .execute(conn)?;
        }

        if let Some(icon) = &request.icon {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::icon.eq(icon))
                .execute(conn)?;
        }

        if let Some(system_prompt) = &request.system_prompt {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::system_prompt.eq(system_prompt))
                .execute(conn)?;
        }

        if let Some(patterns) = &request.reasoning_patterns {
            diesel::update(agents::table)
                .filter(agents::id.eq(id))
                .set(agents::reasoning_patterns.eq(patterns))
                .execute(conn)?;
        }

        // Get the updated agent
        let updated_agent = agents::table
            .filter(agents::id.eq(id))
            .first::<Agent>(conn)?;

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
            return Err(AppError::NotFoundError(format!("Agent not found: {}", id)));
        }

        info!("Deleted agent {}", id);
        Ok(())
    }
}
