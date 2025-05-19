use crate::db::DbPool;
use crate::models::amber_store::{
    NewAmberStore, NewAmberStorePayload, UpdateAmberStore,
};
use crate::services::amber_store_service::AmberStoreService;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::utils::encryption::hash_secure_key;

pub async fn create_amber_store(
    pool: web::Data<DbPool>,
    new_amber_store_payload: web::Json<NewAmberStorePayload>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    let secure_key_hash = hash_secure_key(&new_amber_store_payload.secure_key_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash secure key"))?;

    let new_amber_store = NewAmberStore {
        user_id,
        name: new_amber_store_payload.name.clone(),
        data: serde_yaml::to_string(&new_amber_store_payload.data).unwrap(),
        secure_key_hash,
    };

    match AmberStoreService::create_amber_store(&pool, new_amber_store) {
        Ok(amber_store) => Ok(HttpResponse::Created().json(amber_store)),
        Err(e) => {
            log::error!("Error creating amber store: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to create amber store"))
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
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    let mut update_data = update_data.into_inner();
    if let Some(secure_key) = update_data.secure_key_hash.take() {
        let secure_key_hash = hash_secure_key(&secure_key)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash secure key"))?;
        update_data.secure_key_hash = Some(secure_key_hash);
    }

    match AmberStoreService::update_amber_store(
        &pool,
        amber_store_id.into_inner(),
        update_data,
        user_id,
    ) {
        Ok(amber_store) => Ok(HttpResponse::Ok().json(amber_store)),
        Err(e) => {
            log::error!("Error updating amber store: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to update amber store"))
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
