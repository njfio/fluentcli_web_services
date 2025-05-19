use crate::db::DbPool;
use crate::error::AppError;
use crate::models::amber_store::{AmberStore, NewAmberStore, UpdateAmberStore};
use diesel::{prelude::*, update};
use uuid::Uuid;

pub struct AmberStoreService;

impl AmberStoreService {
    pub fn create_amber_store(pool: &DbPool, new_amber_store: NewAmberStore) -> Result<AmberStore, AppError> {
        use crate::schema::amber_store::dsl::*;
        let conn = &mut pool.get()?;
        diesel::insert_into(amber_store)
            .values(&new_amber_store)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn list_amber_stores(pool: &DbPool, user_id: Uuid) -> Result<Vec<AmberStore>, AppError> {
        use crate::schema::amber_store::dsl::*;
        let conn = &mut pool.get()?;
        amber_store.filter(user_id.eq(user_id)).load::<AmberStore>(conn).map_err(AppError::DatabaseError)
    }

    pub fn get_amber_store(pool: &DbPool, amber_store_id: Uuid, user_id: Uuid) -> Result<AmberStore, AppError> {
        use crate::schema::amber_store::dsl::*;
        let conn = &mut pool.get()?;
        amber_store.filter(id.eq(amber_store_id).and(user_id.eq(user_id))).first(conn).map_err(AppError::DatabaseError)
    }

    pub fn update_amber_store(pool: &DbPool, amber_store_id: Uuid, update_data: UpdateAmberStore, user_id: Uuid) -> Result<AmberStore, AppError> {
        use crate::schema::amber_store::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(amber_store.filter(id.eq(amber_store_id).and(user_id.eq(user_id))))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_amber_store(pool: &DbPool, amber_store_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::amber_store::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(amber_store.filter(id.eq(amber_store_id).and(user_id.eq(user_id))))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
}