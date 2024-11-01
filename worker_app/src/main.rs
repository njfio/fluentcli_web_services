use actix_web::{web, App, HttpServer};
use env_logger;
use log::info;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting worker application");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/computer-use")
                .route("/bash", web::post().to(handlers::computer_use::handle_bash))
                .route(
                    "/editor",
                    web::post().to(handlers::computer_use::handle_editor),
                )
                .route(
                    "/computer",
                    web::post().to(handlers::computer_use::handle_computer),
                ),
        )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
