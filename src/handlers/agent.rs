use crate::db::DbPool;
use crate::error::AppError;
use crate::models::agent::{CreateAgentRequest, UpdateAgentRequest};
use crate::services::agent_service::AgentService;
use crate::utils::extractors::AuthenticatedUser;
use actix_web::{web, HttpResponse};
use uuid::Uuid;
use log::{debug, error, info};

pub async fn create_agent(
    pool: web::Data<DbPool>,
    req: web::Json<CreateAgentRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    info!("Received create agent request from user {}", user.0);
    info!("Agent request data: {:?}", req);
    info!("Request method: POST");

    let agent = web::block(move || {
        AgentService::create_agent(&pool, user.0, req.into_inner())
    })
    .await
    .map_err(|e| {
        error!("Error creating agent: {:?}", e);
        AppError::InternalServerError
    })?;

    info!("Agent created successfully");

    // Add CORS headers explicitly
    let mut response = HttpResponse::Created();
    response.append_header(("Access-Control-Allow-Origin", "*"));
    response.append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
    response.append_header(("Access-Control-Allow-Headers", "Content-Type, Authorization"));

    Ok(response.json(agent?))
}

pub async fn get_agent(
    pool: web::Data<DbPool>,
    agent_id: web::Path<Uuid>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    debug!("Received get agent request for agent {} from user {}", agent_id, user.0);

    let agent = web::block(move || {
        AgentService::get_agent(&pool, agent_id.into_inner(), user.0)
    })
    .await
    .map_err(|e| {
        error!("Error getting agent: {:?}", e);
        AppError::InternalServerError
    })?;

    debug!("Agent retrieved successfully");
    Ok(HttpResponse::Ok().json(agent?))
}

pub async fn list_agents(
    pool: web::Data<DbPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    info!("Received list agents request from user {}", user.0);
    info!("Request method: GET");

    let agents = web::block(move || AgentService::list_agents(&pool, user.0))
        .await
        .map_err(|e| {
            error!("Error listing agents: {:?}", e);
            AppError::InternalServerError
        })?;

    info!("Agents listed successfully");

    // Add CORS headers explicitly
    let mut response = HttpResponse::Ok();
    response.append_header(("Access-Control-Allow-Origin", "*"));
    response.append_header(("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
    response.append_header(("Access-Control-Allow-Headers", "Content-Type, Authorization"));

    Ok(response.json(agents?))
}

pub async fn update_agent(
    pool: web::Data<DbPool>,
    agent_id: web::Path<Uuid>,
    req: web::Json<UpdateAgentRequest>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    debug!("Received update agent request for agent {} from user {}", agent_id, user.0);

    let agent = web::block(move || {
        AgentService::update_agent(&pool, agent_id.into_inner(), user.0, req.into_inner())
    })
    .await
    .map_err(|e| {
        error!("Error updating agent: {:?}", e);
        AppError::InternalServerError
    })?;

    info!("Agent updated successfully");
    Ok(HttpResponse::Ok().json(agent?))
}

pub async fn delete_agent(
    pool: web::Data<DbPool>,
    agent_id: web::Path<Uuid>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    debug!("Received delete agent request for agent {} from user {}", agent_id, user.0);

    web::block(move || AgentService::delete_agent(&pool, agent_id.into_inner(), user.0))
        .await
        .map_err(|e| {
            error!("Error deleting agent: {:?}", e);
            AppError::InternalServerError
        })??;

    info!("Agent deleted successfully");
    Ok(HttpResponse::NoContent().finish())
}

// Handle OPTIONS requests for CORS preflight
pub async fn options_handler() -> HttpResponse {
    info!("Received OPTIONS request for agent endpoint");

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type, Authorization"))
        .finish()
}
