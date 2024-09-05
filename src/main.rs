mod db;
mod error;
mod models;
mod schema;
mod routes;
mod handlers;
mod services;
mod utils;
use dotenv::dotenv;

use actix_web::{App, HttpServer, web};
use db::{create_db_pool, setup_database};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug"); //remove at release
    dotenv().ok();
    env_logger::init();

    // Set up the database
    let pool = create_db_pool().expect("Failed to create database pool");
    setup_database(&pool).expect("Failed to set up database");
    println!("Database setup complete");

    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(configure_routes())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}