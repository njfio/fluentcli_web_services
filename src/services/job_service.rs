use crate::db::DbPool;
use crate::error::AppError;
use crate::models::job::{Job, NewJob, UpdateJob};
use diesel::prelude::*;
use uuid::Uuid;

pub struct JobService;

impl JobService {
    pub fn create_job(pool: &DbPool, new_job: NewJob) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new job into database: {:?}", new_job);
        match diesel::insert_into(jobs)
            .values(&new_job)
            .get_result(conn) {
            Ok(job) => {
                log::info!("Job inserted into database: {:?}", job);
                Ok(job)
            },
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
        log::info!("Getting job with id: {:?} for user_id: {:?}", job_id, user_id);
        jobs.filter(id.eq(job_id).and(user_id.eq(user_id)))
            .first::<Job>(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_job(pool: &DbPool, job_id: Uuid, update_data: UpdateJob, user_id: Uuid) -> Result<Job, AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Updating job with id: {:?} for user_id: {:?}", job_id, user_id);
        diesel::update(jobs.filter(id.eq(job_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::jobs::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Deleting job with id: {:?} for user_id: {:?}", job_id, user_id);
        diesel::delete(jobs.filter(id.eq(job_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map(|_| ())
            .map_err(AppError::DatabaseError)
    }

    pub fn start_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<Job, AppError> {
        // Implement the logic to start a job
        unimplemented!()
    }

    pub fn stop_job(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<Job, AppError> {
        // Implement the logic to stop a job
        unimplemented!()
    }

    pub fn get_job_status(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<String, AppError> {
        // Implement the logic to get the status of a job
        unimplemented!()
    }

    pub fn get_job_output(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<String, AppError> {
        // Implement the logic to get the output of a job
        unimplemented!()
    }

    pub fn get_job_logs(pool: &DbPool, job_id: Uuid, user_id: Uuid) -> Result<String, AppError> {
        // Implement the logic to get the logs of a job
        unimplemented!()
    }
}