use actix_web::{web, HttpResponse, Responder};

pub async fn create_amber_store() -> impl Responder {
    HttpResponse::Ok().body("Create amber store")
}

pub async fn list_amber_stores() -> impl Responder {
    HttpResponse::Ok().body("List amber stores")
}

pub async fn get_amber_store() -> impl Responder {
    HttpResponse::Ok().body("Get amber store")
}

pub async fn update_amber_store() -> impl Responder {
    HttpResponse::Ok().body("Update amber store")
}

pub async fn delete_amber_store() -> impl Responder {
    HttpResponse::Ok().body("Delete amber store")
}
