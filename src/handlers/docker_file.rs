use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::docker_file_service::DockerFileService;
use crate::models::docker_file::{NewDockerFile, UpdateDockerFile, NewDockerFilePayload};

pub async fn create_docker_file(
    pool: web::Data<DbPool>,
    new_docker_file_payload: web::Json<NewDockerFilePayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating docker file for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_docker_file_payload);

    let new_docker_file = NewDockerFile {
        user_id,
        name: new_docker_file_payload.name.clone(),
        content: new_docker_file_payload.content.clone(),
    };
    match DockerFileService::create_docker_file(&pool, new_docker_file) {
        Ok(docker_file) => {
            log::info!("Docker file created successfully: {:?}", docker_file);
            HttpResponse::Created().json(docker_file)
        },
        Err(e) => {
            log::error!("Error creating docker file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create docker file")
        }
    }
}

pub async fn list_docker_files(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match DockerFileService::list_docker_files(&pool, user_id) {
        Ok(docker_files) => HttpResponse::Ok().json(docker_files),
        Err(e) => {
            log::error!("Error listing docker files: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list docker files")
        }
    }
}

pub async fn get_docker_file(
    pool: web::Data<DbPool>,
    docker_file_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match DockerFileService::get_docker_file(&pool, docker_file_id.into_inner(), user_id) {
        Ok(docker_file) => HttpResponse::Ok().json(docker_file),
        Err(e) => {
            log::error!("Error getting docker file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get docker file")
        }
    }
}

pub async fn update_docker_file(
    pool: web::Data<DbPool>,
    docker_file_id: web::Path<Uuid>,
    update_data: web::Json<UpdateDockerFile>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match DockerFileService::update_docker_file(&pool, docker_file_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(docker_file) => HttpResponse::Ok().json(docker_file),
        Err(e) => {
            log::error!("Error updating docker file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update docker file")
        }
    }
}

pub async fn delete_docker_file(
    pool: web::Data<DbPool>,
    docker_file_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match DockerFileService::delete_docker_file(&pool, docker_file_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting docker file: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete docker file")
        }
    }
}