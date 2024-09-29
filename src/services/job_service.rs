use crate::db::DbPool;
use crate::error::AppError;
use crate::handlers::user;
use crate::models::fluentcli::CommandRequest;
use crate::models::job::{Job, NewJob, UpdateJob};
use crate::services::fluentcli_service::FluentCLIService;
use crate::services::pipeline_service::PipelineService;
use diesel::prelude::*;
use serde_json;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use tokio;
use uuid::Uuid;
pub struct JobService;

impl JobService {
    pub fn create_job(pool: &DbPool, new_job: NewJob) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new job into database: {:?}", new_job);
        match diesel::insert_into(jobs).values(&new_job).get_result(conn) {
            Ok(job) => {
                log::info!("Job inserted into database: {:?}", job);
                Ok(job)
            }
            Err(e) => {
                log::error!("Error inserting job into database: {:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn list_jobs(pool: &DbPool, user_id: Uuid) -> Result<Vec<Job>, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Listing jobs for user_id: {:?}", user_id);
        jobs.filter(user_id.eq(user_id))
            .load::<Job>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn get_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!(
            "Getting job with id: {:?} for user_id: {:?}",
            job_id,
            user_id
        );
        jobs.filter(id.eq(job_id).and(user_id.eq(user_id)))
            .first::<Job>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_job(
        pool: &DbPool,
        job_id: Uuid,
        update_data: UpdateJob,
        user_id: Uuid,
    ) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!(
            "Updating job with id: {:?} for user_id: {:?}",
            job_id,
            user_id
        );
        diesel::update(jobs.filter(id.eq(job_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!(
            "Deleting job with id: {:?} for user_id: {:?}",
            job_id,
            user_id
        );
        diesel::delete(jobs.filter(id.eq(job_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map(|_| ())
            .map_err(AppError::DatabaseError)
    }

    pub fn pipeline_exists(
        pool: &DbPool,
        pipeline_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        let exists = diesel::select(diesel::dsl::exists(
            pipelines
                .filter(id.eq(pipeline_id))
                .filter(user_id.eq(user_id)),
        ))
        .get_result(conn)?;
        Ok(exists)
    }

    pub async fn start_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        // Fetch the job
        let job = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .first::<Job>(conn)?;

        // Check if the job is already running
        if job.status == "running" {
            return Err(AppError::BadRequest("Job is already running".to_string()));
        }

        // Fetch the pipeline content
        let pipeline_content =
            PipelineService::fetch_pipeline_content(pool, job.pipeline_id, job.user_id)?;

        // Get the shared temporary path from environment variable
        let shared_tmp_path =
            std::env::var("SHARED_TMP_PATH").map_err(|e| AppError::EnvVarError(e))?;

        // Create a temporary file with the pipeline content in the shared path
        // Create a temporary file with the pipeline content in the shared path
        let temp_file_path = format!("{}/pipeline_{}.yaml", shared_tmp_path, job_id);
        log::debug!("Creating temp file at: {}", temp_file_path);
        std::fs::write(
            &temp_file_path,
            pipeline_content.trim_end().trim_matches('"'),
        )
        .map_err(|e| AppError::TempFileError(format!("Failed to write file: {}", e)))?;
        // Verify the file was created
        if std::path::Path::new(&temp_file_path).exists() {
            log::debug!("Temp file successfully created at: {}", temp_file_path);
        } else {
            log::error!("Failed to create temp file at: {}", temp_file_path);
            return Err(AppError::TempFileError(
                "Failed to create temporary file".to_string(),
            ));
        }

        // Update job status to "running"
        let updated_job = diesel::update(jobs.find(job_id))
            .set((status.eq("running"), started_at.eq(diesel::dsl::now)))
            .get_result::<Job>(conn)?;

        // Execute the job using FluentCLIService
        let command_request = CommandRequest {
            command: "openai".to_string(),
            args: vec![
                "pipeline".to_string(),
                "--file".to_string(),
                temp_file_path.clone(),
                "--input".to_string(),
                job.data_path.unwrap_or_default(),
                "--run-id".to_string(),
                job.id.to_string(),
                "--json-output".to_string(),
            ],
        };

        // Clone the pool for use in the spawned task
        let pool_clone = pool.clone();
        let job_id_clone = job.id;

        // Execute the command asynchronously
        tokio::spawn(async move {
            let result = FluentCLIService::execute_command(job.user_id, command_request).await;

            if let Ok(mut conn) = pool_clone.get() {
                match result {
                    Ok(command_result) => {
                        // Read the state file content
                        let state_file_path =
                            format!("{}/state_file_{}.json", shared_tmp_path, job_id_clone);
                        let file_content = std::fs::read_to_string(&state_file_path)
                            .unwrap_or_else(|_| "{}".to_string());

                        // Update job with results and state file content
                        let _ = diesel::update(jobs.find(job_id_clone))
                            .set((
                                status.eq("completed"),
                                completed_at.eq(diesel::dsl::now),
                                results.eq(serde_json::to_value(command_result)
                                    .unwrap_or(serde_json::Value::Null)),
                                state_file_content
                                    .eq(serde_json::from_str::<serde_json::Value>(&file_content)
                                        .unwrap_or(serde_json::Value::Null)),
                            ))
                            .execute(&mut conn);
                        // Add a delay before file deletion (e.g., 10 seconds)
                        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

                        if std::path::Path::new(&temp_file_path).exists() {
                            log::debug!(
                                "Temp file still exists before deletion: {}",
                                temp_file_path
                            );
                        } else {
                            log::warn!(
                                "Temp file no longer exists before deletion: {}",
                                temp_file_path
                            );
                        }
                        // Clean up the temporary files
                        let _ = std::fs::remove_file(&temp_file_path);
                        let _ = std::fs::remove_file(&state_file_path);
                    }
                    Err(e) => {
                        // Update job with error status
                        let _ = diesel::update(jobs.find(job_id_clone))
                            .set((
                                status.eq("failed"),
                                completed_at.eq(diesel::dsl::now),
                                results.eq(serde_json::to_value(e.to_string())
                                    .unwrap_or(serde_json::Value::Null)),
                            ))
                            .execute(&mut conn);

                        // Clean up the temporary file
                        let _ = std::fs::remove_file(&temp_file_path);
                    }
                }
            }
        });

        Ok(updated_job)
    }

    pub async fn stop_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        // Fetch the job
        let job = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .first::<Job>(conn)?;

        // Check if the job is running
        if job.status != "running" {
            return Err(AppError::BadRequest("Job is not running".to_string()));
        }

        // Update job status to "stopped"
        let updated_job = diesel::update(jobs.find(job_id))
            .set((status.eq("stopped"), completed_at.eq(diesel::dsl::now)))
            .get_result::<Job>(conn)?;

        // TODO: Implement actual job stopping mechanism (e.g., sending a signal to the running process)

        Ok(updated_job)
    }

    pub async fn get_job_status(
        pool: &DbPool,
        job_id: Uuid,
        user_id: Uuid,
    ) -> Result<String, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        let job_status = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .select(status)
            .first::<String>(conn)?;

        Ok(job_status)
    }

    pub async fn get_job_output(
        pool: &DbPool,
        job_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<serde_json::Value>, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        let job_results = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .select(results)
            .first::<Option<serde_json::Value>>(conn)?;

        Ok(job_results)
    }

    pub async fn get_job_logs(
        pool: &DbPool,
        job_id: Uuid,
        user_id: Uuid,
    ) -> Result<String, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        let job = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .first::<Job>(conn)?;

        // TODO: Implement actual log fetching mechanism
        // For now, we'll return a placeholder message
        Ok(format!("Logs for job {} are not yet implemented", job_id))
    }
}
