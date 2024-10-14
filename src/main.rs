mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;
use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use log::debug;

use db::{create_db_pool, setup_database};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,actix_web=debug,actix_cors=trace,fluentcli_web_services=debug",
    );
    dotenv().ok();
    env_logger::init();

    // Set up the database
    let pool = create_db_pool().expect("Failed to create database pool");
    setup_database(&pool).expect("Failed to set up database");
    println!("Database setup complete");

    HttpServer::new(move || {
        let cors = Cors::permissive().supports_credentials().max_age(3600);

        debug!("CORS configuration: {:?}", cors);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(configure_routes())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
