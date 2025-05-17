use crate::db::DbPool;
use crate::models::job::Job;
use crate::services::job_service::JobService;
use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub struct JobScheduler;

impl JobScheduler {
    pub fn start(pool: DbPool) {
        tokio::spawn(async move {
            loop {
                if let Ok(jobs) = JobService::fetch_scheduled_jobs(&pool) {
                    for job in jobs {
                        if is_due(&job) {
                            let _ = JobService::start_job(&pool, job.id, job.user_id).await;
                        }
                    }
                }
                sleep(Duration::from_secs(60)).await;
            }
        });
    }
}

fn is_due(job: &Job) -> bool {
    if let Some(Value::Object(map)) = &job.timers {
        if let Some(Value::String(ts)) = map.get("run_at") {
            if let Ok(run_at) = DateTime::parse_from_rfc3339(ts) {
                return run_at <= Utc::now();
            }
        }
    }
    false
}
