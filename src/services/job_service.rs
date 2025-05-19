use crate::db::DbPool;
use crate::error::AppError;
use crate::handlers::user;
use crate::models::fluentcli::CommandRequest;
use crate::models::job::{Job, NewJob, UpdateJob};

use crate::services::fluentcli_service::FluentCLIService;
use crate::services::pipeline_service::PipelineService;
use diesel::prelude::*;
use serde_json::json;
use std::fmt::Debug;
use tempfile::NamedTempFile;
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};
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

    pub fn fetch_scheduled_jobs(pool: &DbPool) -> Result<Vec<Job>, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        jobs.filter(status.eq("scheduled"))
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

    pub fn schedule_job(
        pool: &DbPool,
        job_id: Uuid,
        user_id: Uuid,
        run_at: String,
    ) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        let timers_json = json!({"run_at": run_at});
        let updated_job = diesel::update(jobs.filter(id.eq(job_id).and(user_id.eq(user_id))))
            .set((status.eq("scheduled"), timers.eq(Some(timers_json))))
            .get_result::<Job>(conn)?;
        Ok(updated_job)
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

        // Parse the YAML to get the pipeline name
        let pipeline_yaml: serde_yaml::Value = serde_yaml::from_str(&pipeline_content)
            .map_err(|e| AppError::YamlParseError(e.to_string()))?;
        let pipeline_name = pipeline_yaml["name"]
            .as_str()
            .ok_or_else(|| AppError::YamlParseError("Pipeline name not found".to_string()))?
            .to_string();

        // Get the shared temporary path from environment variable
        let shared_tmp_path =
            std::env::var("SHARED_TMP_PATH").map_err(|e| AppError::EnvVarError(e))?;

        // Get the Fluent state store path from environment variable
        let fluent_state_store =
            std::env::var("FLUENT_STATE_STORE").map_err(|e| AppError::EnvVarError(e))?;

        // Create a temporary file with the pipeline content in the shared path
        let temp_file_path = format!("{}/pipeline_{}.yaml", shared_tmp_path, job_id);
        log::debug!("Creating temp file at: {}", temp_file_path);
        tokio::fs::write(
            &temp_file_path,
            pipeline_content.trim_end().trim_matches('"'),
        )
        .await
        .map_err(|e| AppError::TempFileError(format!("Failed to write file: {}", e)))?;

        // Verify the file was created
        if tokio::fs::metadata(&temp_file_path).await.is_ok() {
            log::debug!("Temp file successfully created at: {}", temp_file_path);
        } else {
            log::error!("Failed to create temp file at: {}", temp_file_path);
            return Err(AppError::TempFileError(
                "Failed to create temp file".to_string(),
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
        let fluent_state_store_clone = fluent_state_store.clone();
        let temp_file_path_clone = temp_file_path.clone();

        tokio::spawn(async move {
            let result = FluentCLIService::execute_command(job.user_id, command_request).await;

            if let Ok(mut conn) = pool_clone.get() {
                let state_file_pattern = format!("{}-{}.json", pipeline_name, job_id_clone);
                log::debug!(
                    "Looking for state file with pattern: {} in directory: {}",
                    state_file_pattern,
                    fluent_state_store_clone
                );

                // Add a delay before reading the state file
                sleep(Duration::from_secs(5)).await;

                let mut attempts = 0;
                const MAX_ATTEMPTS: u32 = 10;
                const RETRY_DELAY: Duration = Duration::from_secs(5);

                let state_file_path = loop {
                    attempts += 1;
                    let mut found_path = None;
                    match tokio::fs::read_dir(&fluent_state_store).await {
                        Ok(mut entries) => {
                            while let Ok(Some(entry)) = entries.next_entry().await {
                                let file_name = entry.file_name();
                                if file_name.to_string_lossy() == state_file_pattern {
                                    found_path = Some(entry.path());
                                    break;
                                }
                            }
                            if let Some(path) = found_path {
                                log::debug!("Found matching state file: {:?}", path);
                                break Some(path);
                            }
                        }
                        Err(e) => {
                            log::error!("Error reading directory: {:?}", e);
                        }
                    }

                    if attempts >= MAX_ATTEMPTS {
                        log::warn!(
                            "No state file found for job {} in directory {} after {} attempts",
                            job_id_clone,
                            fluent_state_store,
                            MAX_ATTEMPTS
                        );
                        break None;
                    }

                    log::debug!(
                        "State file not found, attempt {} of {}",
                        attempts,
                        MAX_ATTEMPTS
                    );
                    sleep(RETRY_DELAY).await;
                };

                let state_file_content_str = if let Some(path) = state_file_path.as_ref() {
                    tokio::fs::read_to_string(path).await.unwrap_or_else(|e| {
                        log::warn!("Failed to read state file {:?}: {}", path, e);
                        "{}".to_string()
                    })
                } else {
                    log::warn!(
                        "No state file found for job {} in directory {}",
                        job_id_clone,
                        fluent_state_store
                    );
                    "{}".to_string()
                };

                let state_file_json =
                    serde_json::from_str::<serde_json::Value>(&state_file_content_str)
                        .unwrap_or_else(|e| {
                            log::warn!("Failed to parse state file as JSON: {}", e);
                            serde_json::Value::Null
                        });

                match result {
                    Ok(command_result) => {
                        let _ = diesel::update(jobs.find(job_id_clone))
                            .set((
                                status.eq("completed"),
                                completed_at.eq(diesel::dsl::now),
                                results.eq(serde_json::to_value(command_result)
                                    .unwrap_or(serde_json::Value::Null)),
                                state_file_content.eq(state_file_json),
                            ))
                            .execute(&mut conn);
                    }
                    Err(e) => {
                        let _ = diesel::update(jobs.find(job_id_clone))
                            .set((
                                status.eq("failed"),
                                completed_at.eq(diesel::dsl::now),
                                results.eq(serde_json::to_value(e.to_string())
                                    .unwrap_or(serde_json::Value::Null)),
                                state_file_content.eq(state_file_json),
                            ))
                            .execute(&mut conn);
                    }
                }

                // Add a delay before file deletion (e.g., 10 seconds)
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;

                // Clean up the temporary files
                let _ = tokio::fs::remove_file(&temp_file_path).await;
                if let Some(path) = state_file_path {
                    let _ = tokio::fs::remove_file(path).await;
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

        // Send stop signal to worker
        let _ = FluentCLIService::stop_command(job.id).await;

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
    ) -> Result<Option<serde_json::Value>, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        let job_results = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .select(results)
            .first::<Option<serde_json::Value>>(conn)?;

        Ok(job_results)
    }

    pub async fn get_job_data(
        pool: &DbPool,
        job_id: Uuid,
        user_id: Uuid,
        filter_key: Option<String>,
    ) -> Result<Option<serde_json::Value>, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;

        let job_state_file_content = jobs
            .filter(id.eq(job_id).and(user_id.eq(user_id)))
            .select(state_file_content)
            .first::<Option<serde_json::Value>>(conn)?;

        if let Some(content) = job_state_file_content {
            if let Some(key) = filter_key {
                if let Some(data) = content.get("data") {
                    if let Some(filtered_value) = data.get(&key) {
                        return Ok(Some(json!({ key: filtered_value })));
                    }
                }
                Ok(None)
            } else {
                Ok(Some(content))
            }
        } else {
            Ok(None)
        }
    }
}
