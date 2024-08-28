use actix_web::{web, HttpResponse, Responder};

pub async fn create_docker_file() -> impl Responder {
    HttpResponse::Ok().body("Create docker file")
}

pub async fn list_docker_files() -> impl Responder {
    HttpResponse::Ok().body("List docker files")
}

pub async fn get_docker_file() -> impl Responder {
    HttpResponse::Ok().body("Get docker file")
}

pub async fn update_docker_file() -> impl Responder {
    HttpResponse::Ok().body("Update docker file")
}

pub async fn delete_docker_file() -> impl Responder {
    HttpResponse::Ok().body("Delete docker file")
}
