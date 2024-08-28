use actix_web::{web, HttpResponse, Responder};

pub async fn create_pipeline() -> impl Responder {
    HttpResponse::Ok().body("Create pipeline")
}

pub async fn list_pipelines() -> impl Responder {
    HttpResponse::Ok().body("List pipelines")
}

pub async fn get_pipeline() -> impl Responder {
    HttpResponse::Ok().body("Get pipeline")
}

pub async fn update_pipeline() -> impl Responder {
    HttpResponse::Ok().body("Update pipeline")
}

pub async fn delete_pipeline() -> impl Responder {
    HttpResponse::Ok().body("Delete pipeline")
}
