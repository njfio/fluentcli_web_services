use crate::db::db::DbPool;
use crate::error::AppError;
use crate::models::agent::{Agent, NewAgent};
use crate::models::agent_session::{AgentSession, NewAgentSession};
use crate::services::{ChatService, LLMService, LLMChatMessage};
use diesel::prelude::*;
use futures::StreamExt;
use log::{error, info};
use std::sync::Arc;
use uuid::Uuid;
use serde_json::Value;

pub struct AgentService;

impl AgentService {
    pub fn create_session(
        pool: &DbPool,
        user_id: Uuid,
        title: String,
    ) -> Result<AgentSession, AppError> {
        use crate::schema::agent_sessions::dsl::*;

        let conversation = ChatService::create_conversation(pool, user_id, title, "chat".to_string())?;
        let new_session = NewAgentSession {
            conversation_id: conversation.id,
        };
        diesel::insert_into(agent_sessions)
            .values(&new_session)
            .get_result::<AgentSession>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn add_agent(
        pool: &DbPool,
        session_id_val: Uuid,
        user_llm_config_id_val: Uuid,
        role_name_val: String,
        tool_config_val: Option<Value>,
    ) -> Result<Agent, AppError> {
        use crate::schema::agents::dsl::*;

        let new_agent = NewAgent {
            session_id: session_id_val,
            user_llm_config_id: user_llm_config_id_val,
            role_name: role_name_val,
            tool_config: tool_config_val,
        };
        diesel::insert_into(agents)
            .values(&new_agent)
            .get_result::<Agent>(&mut pool.get()?)
            .map_err(AppError::DatabaseError)
    }

    pub fn end_session(pool: &DbPool, session_id_val: Uuid) -> Result<(), AppError> {
        use crate::schema::agent_sessions::dsl::*;
        diesel::delete(agent_sessions.filter(id.eq(session_id_val)))
            .execute(&mut pool.get()?)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub async fn run_step(pool: Arc<DbPool>, session_id_val: Uuid) -> Result<(), AppError> {
        use crate::schema::agents::dsl::*;
        use crate::schema::agent_sessions::dsl as sessions_dsl;

        let conn = &mut pool.get()?;
        let session: AgentSession = sessions_dsl::agent_sessions
            .find(session_id_val)
            .first(conn)
            .map_err(AppError::DatabaseError)?;

        let agent_list = agents
            .filter(session_id.eq(session_id_val))
            .load::<Agent>(conn)
            .map_err(AppError::DatabaseError)?;

        let messages = ChatService::get_messages(&pool, session.conversation_id)?;
        let llm_messages: Vec<LLMChatMessage> = messages
            .into_iter()
            .map(|m| LLMChatMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        for ag in agent_list {
            let user_config = ChatService::get_user_llm_config_by_id(&pool, ag.user_llm_config_id)?;
            let provider = ChatService::get_llm_provider(&pool, user_config.provider_id)?;
            let stream = LLMService::llm_stream_chat(pool.clone(), Arc::new(provider.clone()), Arc::new(user_config.clone()), llm_messages.clone()).await;
            let chunks: Result<Vec<String>, _> = stream.collect::<Vec<_>>().await.into_iter().collect();
            let content = chunks?.join("");
            ChatService::create_message_with_attachments(
                &pool,
                session.conversation_id,
                ag.role_name.clone(),
                content,
                provider.name,
                None,
                None,
            )
            .await?;
        }
        Ok(())
    }
}
