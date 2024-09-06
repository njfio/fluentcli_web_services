use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use uuid::Uuid;
use crate::db::DbPool;
use crate::services::pipeline_service::PipelineService;
use crate::models::pipeline::{NewPipeline, UpdatePipeline, NewPipelinePayload};

pub async fn create_pipeline(
    pool: web::Data<DbPool>,
    new_pipeline_payload: web::Json<NewPipelinePayload>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    log::info!("Creating pipeline for user_id: {}", user_id);
    log::info!("Received data: {:?}", new_pipeline_payload);

    let new_pipeline = NewPipeline {
        user_id,
        name: new_pipeline_payload.name.clone(),
        data: new_pipeline_payload.data.clone(),
    };
    match PipelineService::create_pipeline(&pool, new_pipeline) {
        Ok(pipeline) => {
            log::info!("Pipeline created successfully: {:?}", pipeline);
            HttpResponse::Created().json(pipeline)
        },
        Err(e) => {
            log::error!("Error creating pipeline: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create pipeline")
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
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match PipelineService::update_pipeline(&pool, pipeline_id.into_inner(), update_data.into_inner(), user_id) {
        Ok(pipeline) => HttpResponse::Ok().json(pipeline),
        Err(e) => {
            log::error!("Error updating pipeline: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update pipeline")
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