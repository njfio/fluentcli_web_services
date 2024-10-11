use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use fws::services::llm_service::LLMService;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = establish_connection();
    let providers = LLMService::get_llm_providers(&pool)?;

    println!("LLM Providers:");
    for provider in providers {
        println!("ID: {}", provider.id);
        println!("Name: {}", provider.name);
        println!("API Endpoint: {}", provider.api_endpoint);
        println!("---");
    }

    Ok(())
}
