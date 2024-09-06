use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::amber_store_service::AmberStoreService;
use crate::models::amber_store::{NewAmberStore, UpdateAmberStore, NewAmberStorePayload};


pub async fn create_amber_store(
    pool: web::Data<DbPool>,
    new_amber_store_payload: web::Json<NewAmberStorePayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating amber store for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_amber_store_payload);

    let new_amber_store = NewAmberStore {
        user_id,
        data: new_amber_store_payload.data.clone(),
    };
    match AmberStoreService::create_amber_store(&pool, new_amber_store) {
        Ok(amber_store) => {
            log::info!("Amber store created successfully: {:?}", amber_store);
            HttpResponse::Created().json(amber_store)
        },
        Err(e) => {
            log::error!("Error creating amber store: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create amber store")
        }
    }
}

pub async fn list_amber_stores(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match AmberStoreService::list_amber_stores(&pool, user_id) {
        Ok(amber_stores) => HttpResponse::Ok().json(amber_stores),
        Err(e) => {
            log::error!("Error listing amber stores: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list amber stores")
        }
    }
}

pub async fn get_amber_store(
    pool: web::Data<DbPool>,
    amber_store_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match AmberStoreService::get_amber_store(&pool, amber_store_id.into_inner(), user_id) {
        Ok(amber_store) => HttpResponse::Ok().json(amber_store),
        Err(e) => {
            log::error!("Error getting amber store: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get amber store")
        }
    }
}

pub async fn update_amber_store(
    pool: web::Data<DbPool>,
    amber_store_id: web::Path<Uuid>,
    update_data: web::Json<UpdateAmberStore>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match AmberStoreService::update_amber_store(&pool, amber_store_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(amber_store) => HttpResponse::Ok().json(amber_store),
        Err(e) => {
            log::error!("Error updating amber store: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update amber store")
        }
    }
}

pub async fn delete_amber_store(
    pool: web::Data<DbPool>,
    amber_store_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match AmberStoreService::delete_amber_store(&pool, amber_store_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting amber store: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete amber store")
        }
    }
}