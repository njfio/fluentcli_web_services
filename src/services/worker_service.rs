use crate::db::DbPool;
use crate::error::AppError;
use crate::models::worker::{Worker, NewWorker, UpdateWorker};
use diesel::prelude::*;
use uuid::Uuid;

pub struct WorkerService;

impl WorkerService {
    pub fn create_worker(pool: &DbPool, new_worker: NewWorker) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new worker into database: {:?}", new_worker);
        match diesel::insert_into(workers)
            .values(&new_worker)
            .get_result(conn) {
            Ok(worker) => {
                log::info!("Worker inserted into database: {:?}", worker);
                Ok(worker)
            },
            Err(e) => {
                log::error!("Error inserting worker into database: {:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub fn list_workers(pool: &DbPool, user_id: Uuid) -> Result<Vec<Worker>, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        workers.filter(user_id.eq(user_id)).load::<Worker>(conn).map_err(AppError::DatabaseError)
    }

    pub fn get_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        workers.filter(id.eq(worker_id).and(user_id.eq(user_id))).first(conn).map_err(AppError::DatabaseError)
    }

    pub fn update_worker(pool: &DbPool, worker_id: Uuid, update_data: UpdateWorker, user_id: Uuid) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub fn activate_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(active.eq(true))
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn deactivate_worker(pool: &DbPool, worker_id: Uuid, user_id: Uuid) -> Result<Worker, AppError> {
        use crate::schema::workers::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(workers.filter(id.eq(worker_id).and(user_id.eq(user_id))))
            .set(active.eq(false))
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }
}