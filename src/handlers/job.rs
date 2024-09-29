use crate::db::DbPool;
use crate::models::job::{NewJob, NewJobPayload, UpdateJob};
use crate::services::job_service::JobService;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

pub async fn create_job(
    pool: web::Data<DbPool>,
    new_job_payload: web::Json<NewJobPayload>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();

    // Convert worker_type from String to Uuid
    let worker_type = Uuid::parse_str(&new_job_payload.worker_type)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid worker_type UUID"))?;

    // Convert state_file_content from Option<String> to Option<Value>
    let state_file_content = new_job_payload
        .state_file_content
        .as_ref()
        .map(|content| {
            serde_json::from_str(content)
                .map_err(|_| actix_web::error::ErrorBadRequest("Invalid state_file_content JSON"))
        })
        .transpose()?;

    let new_job = NewJob {
        user_id,
        uri: Uuid::new_v4(), // Generate a new UUID for uri
        config: new_job_payload.config,
        amber_id: new_job_payload.amber_id,
        state_file_content,
        data_path: new_job_payload.data_path.clone(),
        worker_type,
        triggers: new_job_payload.triggers.clone(),
        timers: new_job_payload.timers.clone(),
        status: new_job_payload.status.clone(),
        pipeline_id: new_job_payload.pipeline_id,
        results: new_job_payload.results.clone(),
    };

    match JobService::create_job(&pool, new_job) {
        Ok(job) => Ok(HttpResponse::Created().json(job)),
        Err(e) => {
            log::error!("Error creating job: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to create job"))
        }
    }
}

pub async fn list_jobs(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::list_jobs(&pool, user_id) {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(e) => {
            log::error!("Error listing jobs: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to list jobs")
        }
    }
}

pub async fn get_job(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::get_job(&pool, job_id.into_inner(), user_id) {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => {
            log::error!("Error getting job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get job")
        }
    }
}

pub async fn update_job(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    update_data: web::Json<UpdateJob>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::update_job(
        &pool,
        job_id.into_inner(),
        update_data.into_inner(),
        user_id,
    ) {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => {
            log::error!("Error updating job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update job")
        }
    }
}

pub async fn delete_job(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::delete_job(&pool, job_id.into_inner(), user_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Error deleting job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete job")
        }
    }
}

pub async fn start_job(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::start_job(&pool, job_id.into_inner(), user_id).await {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => {
            log::error!("Error starting job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to start job")
        }
    }
}

pub async fn stop_job(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::stop_job(&pool, job_id.into_inner(), user_id).await {
        Ok(job) => HttpResponse::Ok().json(job),
        Err(e) => {
            log::error!("Error stopping job: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to stop job")
        }
    }
}

pub async fn get_job_status(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::get_job_status(&pool, job_id.into_inner(), user_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => {
            log::error!("Error getting job status: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get job status")
        }
    }
}

pub async fn get_job_output(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::get_job_output(&pool, job_id.into_inner(), user_id).await {
        Ok(output) => HttpResponse::Ok().json(output),
        Err(e) => {
            log::error!("Error getting job output: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get job output")
        }
    }
}

pub async fn get_job_logs(
    pool: web::Data<DbPool>,
    job_id: web::Path<Uuid>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = req.extensions().get::<Uuid>().cloned().unwrap();
    match JobService::get_job_logs(&pool, job_id.into_inner(), user_id).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => {
            log::error!("Error getting job logs: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get job logs")
        }
    }
}
