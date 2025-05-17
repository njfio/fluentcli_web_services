use crate::models::agent::{Agent, NewAgent, UpdateAgent};
use crate::services::agent_service::AgentService;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn create_agent(new_agent: web::Json<NewAgent>) -> impl Responder {
    let agent = AgentService::create_agent(new_agent.into_inner());
    HttpResponse::Created().json(agent)
}

pub async fn list_agents() -> impl Responder {
    let agents = AgentService::list_agents();
    HttpResponse::Ok().json(agents)
}

pub async fn get_agent(path: web::Path<Uuid>) -> impl Responder {
    match AgentService::get_agent(path.into_inner()) {
        Some(agent) => HttpResponse::Ok().json(agent),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_agent(
    path: web::Path<Uuid>,
    update: web::Json<UpdateAgent>,
) -> impl Responder {
    match AgentService::update_agent(path.into_inner(), update.into_inner()) {
        Some(agent) => HttpResponse::Ok().json(agent),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_agent(path: web::Path<Uuid>) -> impl Responder {
    if AgentService::delete_agent(path.into_inner()) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
