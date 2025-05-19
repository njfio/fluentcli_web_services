use crate::db::DbPool;
use crate::error::AppError;
use crate::models::configuration::{Configuration, NewConfiguration, UpdateConfiguration};
use diesel::prelude::*;
use uuid::Uuid;

pub struct ConfigurationService;

impl ConfigurationService {
    pub fn create_configuration(pool: &DbPool, new_configuration: NewConfiguration) -> Result<Configuration, AppError> {
        use crate::schema::configurations::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Inserting new configuration into database: {:?}", new_configuration);
        diesel::insert_into(configurations)
            .values(&new_configuration)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_configurations(pool: &DbPool, user_id: Uuid) -> Result<Vec<Configuration>, AppError> {
        use crate::schema::configurations::dsl::*;
        let conn = &mut pool.get()?;
        configurations.filter(user_id.eq(user_id)).load::<Configuration>(conn).map_err(AppError::DatabaseError)
    }

    pub fn get_configuration(pool: &DbPool, configuration_id: Uuid, user_id: Uuid) -> Result<Configuration, AppError> {
        use crate::schema::configurations::dsl::*;
        let conn = &mut pool.get()?;
        configurations.filter(id.eq(configuration_id).and(user_id.eq(user_id))).first(conn).map_err(AppError::DatabaseError)
    }

    pub fn update_configuration(pool: &DbPool, configuration_id: Uuid, update_data: UpdateConfiguration, user_id: Uuid) -> Result<Configuration, AppError> {
        use crate::schema::configurations::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(configurations.filter(id.eq(configuration_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_configuration(pool: &DbPool, configuration_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::configurations::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(configurations.filter(id.eq(configuration_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}