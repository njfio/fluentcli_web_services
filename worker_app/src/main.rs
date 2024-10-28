use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use log::{debug, info};
use std::io;

mod computer_use;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    info!("Starting worker app");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(
                web::scope("/computer-use")
                    .route("/health", web::get().to(health_check))
                    .route(
                        "/computer",
                        web::post().to(computer_use::handle_computer_request),
                    )
                    .route(
                        "/text-editor",
                        web::post().to(computer_use::handle_text_editor_request),
                    )
                    .route("/bash", web::post().to(computer_use::handle_bash_request)),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
