use actix_web::{web, HttpResponse, Responder};

pub async fn create_configuration() -> impl Responder {
    HttpResponse::Ok().body("Create configuration")
}

pub async fn list_configurations() -> impl Responder {
    HttpResponse::Ok().body("List configurations")
}

pub async fn get_configuration() -> impl Responder {
    HttpResponse::Ok().body("Get configuration")
}

pub async fn update_configuration() -> impl Responder {
    HttpResponse::Ok().body("Update configuration")
}

pub async fn delete_configuration() -> impl Responder {
    HttpResponse::Ok().body("Delete configuration")
}
