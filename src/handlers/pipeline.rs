use crate::db::DbPool;
use crate::models::pipeline::{NewPipeline, NewPipelinePayload, UpdatePipeline};
use crate::services::pipeline_service::PipelineService;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_yaml;
use uuid::Uuid;

pub async fn create_pipeline(
    pool: web::Data<DbPool>,
    new_pipeline_payload: web::Json<NewPipelinePayload>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    let yaml_data = serde_yaml::to_string(&new_pipeline_payload.data)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid YAML data"))?;

    let new_pipeline = NewPipeline {
        user_id,
        name: new_pipeline_payload.name.clone(),
        data: yaml_data,
    };

    match PipelineService::create_pipeline(&pool, new_pipeline) {
        Ok(pipeline) => Ok(HttpResponse::Created().json(pipeline)),
        Err(e) => {
            log::error!("Error creating pipeline: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to create pipeline"))
        }
    }
}

pub async fn list_pipelines(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match PipelineService::list_pipelines(&pool, user_id) {
        Ok(pipelines) => HttpResponse::Ok().json(pipelines),
        Err(e) => {
            log::error!("Error listing pipelines: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list pipelines")
        }
    }
}

pub async fn get_pipeline(
    pool: web::Data<DbPool>,
    pipeline_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match PipelineService::get_pipeline(&pool, pipeline_id.into_inner(), user_id) {
        Ok(pipeline) => HttpResponse::Ok().json(pipeline),
        Err(e) => {
            log::error!("Error getting pipeline: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get pipeline")
        }
    }
}

pub async fn update_pipeline(
    pool: web::Data<DbPool>,
    pipeline_id: web::Path<Uuid>,
    update_data: web::Json<UpdatePipeline>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    let yaml_data = if let Some(data) = &update_data.data {
        Some(
            serde_yaml::to_string(data)
                .map_err(|_| actix_web::error::ErrorBadRequest("Invalid YAML data"))?,
        )
    } else {
        None
    };

    let update_pipeline = UpdatePipeline {
        name: update_data.name.clone(),
        data: yaml_data,
    };

    match PipelineService::update_pipeline(
        &pool,
        pipeline_id.into_inner(),
        update_pipeline,
        user_id,
    ) {
        Ok(pipeline) => Ok(HttpResponse::Ok().json(pipeline)),
        Err(e) => {
            log::error!("Error updating pipeline: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to update pipeline"))
        }
    }
}

pub async fn delete_pipeline(
    pool: web::Data<DbPool>,
    pipeline_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match PipelineService::delete_pipeline(&pool, pipeline_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting pipeline: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete pipeline")
        }
    }
}
