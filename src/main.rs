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

use db::{create_db_pool, setup_database};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,fluentcli_web_services=debug"); // Updated this line
    dotenv().ok();
    env_logger::init();

    // Set up the database
    let pool = create_db_pool().expect("Failed to create database pool");
    setup_database(&pool).expect("Failed to set up database");
    println!("Database setup complete");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Be cautious with this in production environments
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

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
