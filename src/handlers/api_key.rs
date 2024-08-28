use actix_web::{web, HttpResponse, Responder};

pub async fn create_api_key() -> impl Responder {
    HttpResponse::Ok().body("Create API key")
}

pub async fn list_api_keys() -> impl Responder {
    HttpResponse::Ok().body("List API keys")
}

pub async fn delete_api_key() -> impl Responder {
    HttpResponse::Ok().body("Delete API key")
}
