use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::worker_service::WorkerService;
use crate::models::worker::{NewWorker, UpdateWorker, NewWorkerPayload};


pub async fn create_worker(
    pool: web::Data<DbPool>,
    new_worker_payload: web::Json<NewWorkerPayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating worker for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_worker_payload);

    let new_worker = NewWorker {
        user_id,
        name: new_worker_payload.name.clone(),
        worker_type: new_worker_payload.worker_type,
        active: false,
    };
    log::info!("New worker data: {:?}", new_worker);

    match WorkerService::create_worker(&pool, new_worker) {
        Ok(worker) => {
            log::info!("Worker created successfully: {:?}", worker);
            HttpResponse::Created().json(worker)
        },
        Err(e) => {
            log::error!("Error creating worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create worker")
        }
    }
}

pub async fn list_workers(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::list_workers(&pool, user_id) {
        Ok(workers) => HttpResponse::Ok().json(workers),
        Err(e) => {
            log::error!("Error listing workers: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list workers")
        }
    }
}

pub async fn get_worker(
    pool: web::Data<DbPool>,
    worker_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::get_worker(&pool, worker_id.into_inner(), user_id) {
        Ok(worker) => HttpResponse::Ok().json(worker),
        Err(e) => {
            log::error!("Error getting worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get worker")
        }
    }
}

pub async fn update_worker(
    pool: web::Data<DbPool>,
    worker_id: web::Path<Uuid>,
    update_data: web::Json<UpdateWorker>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::update_worker(&pool, worker_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(worker) => HttpResponse::Ok().json(worker),
        Err(e) => {
            log::error!("Error updating worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update worker")
        }
    }
}

pub async fn delete_worker(
    pool: web::Data<DbPool>,
    worker_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::delete_worker(&pool, worker_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete worker")
        }
    }
}

pub async fn activate_worker(
    pool: web::Data<DbPool>,
    worker_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::activate_worker(&pool, worker_id.into_inner(), user_id) {
        Ok(worker) => HttpResponse::Ok().json(worker),
        Err(e) => {
            log::error!("Error activating worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to activate worker")
        }
    }
}

pub async fn deactivate_worker(
    pool: web::Data<DbPool>,
    worker_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match WorkerService::deactivate_worker(&pool, worker_id.into_inner(), user_id) {
        Ok(worker) => HttpResponse::Ok().json(worker),
        Err(e) => {
            log::error!("Error deactivating worker: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to deactivate worker")
        }
    }
}