use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::secure_vault_service::SecureVaultService;
use crate::models::secure_vault::{NewSecureVault, UpdateSecureVault, NewSecureVaultPayload};
use crate::utils::encryption::encrypt_data;

pub async fn create_secure_vault(
    pool: web::Data<DbPool>,
    new_secure_vault_payload: web::Json<NewSecureVaultPayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating secure vault for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_secure_vault_payload);

    let encrypted_data = encrypt_data(&new_secure_vault_payload.data);
    let new_secure_vault = NewSecureVault {
        user_id,
        name: new_secure_vault_payload.name.clone(),
        encrypted_data,
    };
    log::info!("New secure vault data: {:?}", new_secure_vault);

    match SecureVaultService::create_secure_vault(&pool, new_secure_vault) {
        Ok(secure_vault) => {
            log::info!("Secure vault created successfully: {:?}", secure_vault);
            HttpResponse::Created().json(secure_vault)
        },
        Err(e) => {
            log::error!("Error creating secure vault: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create secure vault")
        }
    }
}

pub async fn list_secure_vaults(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match SecureVaultService::list_secure_vaults(&pool, user_id) {
        Ok(secure_vaults) => HttpResponse::Ok().json(secure_vaults),
        Err(e) => {
            log::error!("Error listing secure vaults: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list secure vaults")
        }
    }
}

pub async fn get_secure_vault(
    pool: web::Data<DbPool>,
    secure_vault_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match SecureVaultService::get_secure_vault(&pool, secure_vault_id.into_inner(), user_id) {
        Ok(secure_vault) => HttpResponse::Ok().json(secure_vault),
        Err(e) => {
            log::error!("Error getting secure vault: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get secure vault")
        }
    }
}

pub async fn update_secure_vault(
    pool: web::Data<DbPool>,
    secure_vault_id: web::Path<Uuid>,
    update_data: web::Json<UpdateSecureVault>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match SecureVaultService::update_secure_vault(&pool, secure_vault_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(secure_vault) => HttpResponse::Ok().json(secure_vault),
        Err(e) => {
            log::error!("Error updating secure vault: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update secure vault")
        }
    }
}

pub async fn delete_secure_vault(
    pool: web::Data<DbPool>,
    secure_vault_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match SecureVaultService::delete_secure_vault(&pool, secure_vault_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting secure vault: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete secure vault")
        }
    }
}