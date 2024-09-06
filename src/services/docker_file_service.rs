use crate::db::DbPool;
use crate::error::AppError;
use crate::models::docker_file::{DockerFile, NewDockerFile, UpdateDockerFile};
use diesel::prelude::*;
use uuid::Uuid;

pub struct DockerFileService;

impl DockerFileService {
    pub fn create_docker_file(pool: &DbPool, new_docker_file: NewDockerFile) -> Result<DockerFile, AppError> {
        use crate::schema::docker_files::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new docker file into database: {:?}", new_docker_file);
        diesel::insert_into(docker_files)
            .values(&new_docker_file)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_docker_files(pool: &DbPool, user_id: Uuid) -> Result<Vec<DockerFile>, AppError> {
        use crate::schema::docker_files::dsl::*;
        let conn = &mut pool.get()?;
        docker_files.filter(user_id.eq(user_id)).load::<DockerFile>(conn).map_err(AppError::DatabaseError)
    }

    pub fn get_docker_file(pool: &DbPool, docker_file_id: Uuid, user_id: Uuid) -> Result<DockerFile, AppError> {
        use crate::schema::docker_files::dsl::*;
        let conn = &mut pool.get()?;
        docker_files.filter(id.eq(docker_file_id).and(user_id.eq(user_id))).first(conn).map_err(AppError::DatabaseError)
    }

    pub fn update_docker_file(pool: &DbPool, docker_file_id: Uuid, update_data: UpdateDockerFile, user_id: Uuid) -> Result<DockerFile, AppError> {
        use crate::schema::docker_files::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(docker_files.filter(id.eq(docker_file_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_docker_file(pool: &DbPool, docker_file_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::docker_files::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(docker_files.filter(id.eq(docker_file_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}