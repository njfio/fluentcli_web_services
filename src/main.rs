mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;
use handlers::metrics;
use services::job_scheduler::JobScheduler;
use crate::config::Config;
use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use log::debug;

use db::{create_db_pool, setup_database};
use routes::configure_routes;

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> Error {
    actix_web::error::ErrorBadRequest(err)
}

fn query_error_handler(
    err: actix_web::error::QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> Error {
    actix_web::error::ErrorBadRequest(err)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,actix_web=debug,actix_cors=trace,fluentcli_web_services=debug",
    );
    dotenv().ok();
    env_logger::init();
    metrics::init_metrics();
    let config = Config::from_env();
    println!("Running in {} mode", config.environment);

    // Set up the database
    let pool = create_db_pool().expect("Failed to create database pool");
    setup_database(&pool).expect("Failed to set up database");
    println!("Database setup complete");

    JobScheduler::start(pool.clone());

    HttpServer::new(move || {
        let allowed_origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| "*".into());
        let mut cors = Cors::default().supports_credentials().max_age(3600);
        if allowed_origins != "*" {
            for origin in allowed_origins.split(',') {
                cors = cors.allowed_origin(origin.trim());
            }
        } else {
            cors = Cors::permissive();
        }

        debug!("CORS configuration: {:?}", cors);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // Add configuration for maximum payload size
            .app_data(
                web::JsonConfig::default()
                    .limit(10485760) // 10MB json payload limit
                    .error_handler(json_error_handler),
            )
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .app_data(web::Data::new(pool.clone()))
            .service(configure_routes())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
