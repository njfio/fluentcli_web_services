use crate::db::db::DbPool;
use crate::error::AppError;
use crate::services::agent_service::AgentService;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub title: String,
}

pub async fn create_session(
    user: AuthenticatedUser,
    pool: web::Data<DbPool>,
    req: web::Json<CreateSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let session = web::block(move || {
        AgentService::create_session(&pool, user.0, req.title.clone())
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;
    Ok(HttpResponse::Created().json(session))
}

#[derive(Deserialize)]
pub struct AddAgentRequest {
    pub user_llm_config_id: Uuid,
    pub role_name: String,
    pub tool_config: Option<Value>,
}

pub async fn add_agent(
    pool: web::Data<DbPool>,
    session_id: web::Path<Uuid>,
    req: web::Json<AddAgentRequest>,
) -> Result<HttpResponse, AppError> {
    let agent = web::block(move || {
        AgentService::add_agent(
            &pool,
            session_id.into_inner(),
            req.user_llm_config_id,
            req.role_name.clone(),
            req.tool_config.clone(),
        )
    })
    .await
    .map_err(|e| AppError::GenericError(Box::new(e)))??;
    Ok(HttpResponse::Created().json(agent))
}

pub async fn run_step(
    pool: web::Data<DbPool>,
    session_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    AgentService::run_step(Arc::new(pool.get_ref().clone()), session_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn end_session(
    pool: web::Data<DbPool>,
    session_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    web::block(move || AgentService::end_session(&pool, session_id.into_inner()))
        .await
        .map_err(|e| AppError::GenericError(Box::new(e)))??;
    Ok(HttpResponse::NoContent().finish())
}
