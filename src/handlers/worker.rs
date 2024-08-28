use actix_web::{web, HttpResponse, Responder};

pub async fn list_workers() -> impl Responder {
    HttpResponse::Ok().body("List workers")
}

pub async fn activate_worker() -> impl Responder {
    HttpResponse::Ok().body("Activate worker")
}

pub async fn deactivate_worker() -> impl Responder {
    HttpResponse::Ok().body("Deactivate worker")
}
