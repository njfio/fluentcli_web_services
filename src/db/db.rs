use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;
use crate::error::AppError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn create_db_pool() -> Result<DbPool, PoolError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn setup_database(pool: &DbPool) -> Result<(), AppError> {
    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}