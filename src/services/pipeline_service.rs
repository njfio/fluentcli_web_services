use crate::db::DbPool;
use crate::error::AppError;
use crate::models::pipeline::{Pipeline, NewPipeline, UpdatePipeline};
use diesel::prelude::*;
use uuid::Uuid;

pub struct PipelineService;

impl PipelineService {
    pub fn create_pipeline(pool: &DbPool, new_pipeline: NewPipeline) -> Result<Pipeline, AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new pipeline into database: {:?}", new_pipeline);
        diesel::insert_into(pipelines)
            .values(&new_pipeline)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_pipelines(pool: &DbPool, user_id: Uuid) -> Result<Vec<Pipeline>, AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        pipelines.filter(user_id.eq(user_id)).load::<Pipeline>(conn).map_err(AppError::DatabaseError)
    }

    pub fn get_pipeline(pool: &DbPool, pipeline_id: Uuid, user_id: Uuid) -> Result<Pipeline, AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        pipelines.filter(id.eq(pipeline_id).and(user_id.eq(user_id))).first(conn).map_err(AppError::DatabaseError)
    }

    pub fn update_pipeline(pool: &DbPool, pipeline_id: Uuid, update_data: UpdatePipeline, user_id: Uuid) -> Result<Pipeline, AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(pipelines.filter(id.eq(pipeline_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_pipeline(pool: &DbPool, pipeline_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::pipelines::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(pipelines.filter(id.eq(pipeline_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}