use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpServer};
use dotenv::dotenv;
use fws::config::Config;
use fws::db::{create_db_pool, setup_database};
use fws::handlers::metrics;
use fws::routes::configure_routes;
use fws::services::job_scheduler::JobScheduler;

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

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting Fluent Web Services server...");

    dotenv().ok();
    env_logger::init();

    metrics::init_metrics();
    let config = Config::from_env();
    println!("Running in {} mode", config.environment);

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
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(
                web::JsonConfig::default()
                    .limit(10485760)
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
