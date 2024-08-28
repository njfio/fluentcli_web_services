use actix_web::{web, HttpResponse, Responder};

pub async fn create_vault_store() -> impl Responder {
    HttpResponse::Ok().body("Create vault store")
}

pub async fn list_vault_stores() -> impl Responder {
    HttpResponse::Ok().body("List vault stores")
}

pub async fn get_vault_store() -> impl Responder {
    HttpResponse::Ok().body("Get vault store")
}

pub async fn update_vault_store() -> impl Responder {
    HttpResponse::Ok().body("Update vault store")
}

pub async fn delete_vault_store() -> impl Responder {
    HttpResponse::Ok().body("Delete vault store")
}
