use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::configuration_service::ConfigurationService;
use crate::models::configuration::{NewConfiguration, UpdateConfiguration, NewConfigurationPayload};

pub async fn create_configuration(
    pool: web::Data<DbPool>,
    new_configuration_payload: web::Json<NewConfigurationPayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating configuration for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_configuration_payload);

    let new_configuration = NewConfiguration {
        user_id,
        name: new_configuration_payload.name.clone(),
        data: new_configuration_payload.data.clone(),
    };
    match ConfigurationService::create_configuration(&pool, new_configuration) {
        Ok(configuration) => {
            log::info!("Configuration created successfully: {:?}", configuration);
            HttpResponse::Created().json(configuration)
        },
        Err(e) => {
            log::error!("Error creating configuration: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create configuration")
        }
    }
}

pub async fn list_configurations(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match ConfigurationService::list_configurations(&pool, user_id) {
        Ok(configurations) => HttpResponse::Ok().json(configurations),
        Err(e) => {
            log::error!("Error listing configurations: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list configurations")
        }
    }
}

pub async fn get_configuration(
    pool: web::Data<DbPool>,
    configuration_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match ConfigurationService::get_configuration(&pool, configuration_id.into_inner(), user_id) {
        Ok(configuration) => HttpResponse::Ok().json(configuration),
        Err(e) => {
            log::error!("Error getting configuration: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get configuration")
        }
    }
}

pub async fn update_configuration(
    pool: web::Data<DbPool>,
    configuration_id: web::Path<Uuid>,
    update_data: web::Json<UpdateConfiguration>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match ConfigurationService::update_configuration(&pool, configuration_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(configuration) => HttpResponse::Ok().json(configuration),
        Err(e) => {
            log::error!("Error updating configuration: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update configuration")
        }
    }
}

pub async fn delete_configuration(
    pool: web::Data<DbPool>,
    configuration_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match ConfigurationService::delete_configuration(&pool, configuration_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting configuration: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete configuration")
        }
    }
}