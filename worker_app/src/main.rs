use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger;
use log::info;

mod handlers;

async fn health_check() -> HttpResponse {
    // Get display dimensions from environment
    let display_width: i32 = std::env::var("DISPLAY_WIDTH")
        .unwrap_or_else(|_| "1024".to_string())
        .parse()
        .unwrap_or(1024);
    let display_height: i32 = std::env::var("DISPLAY_HEIGHT")
        .unwrap_or_else(|_| "768".to_string())
        .parse()
        .unwrap_or(768);
    let display_number: i32 = std::env::var("DISPLAY_NUMBER")
        .unwrap_or_else(|_| "99".to_string())
        .parse()
        .unwrap_or(99);

    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "display": {
            "width_px": display_width,
            "height_px": display_height,
            "number": display_number,
            "env": format!(":{}",display_number)
        },
        "tools": [
            {
                "type": "computer_20241022",
                "name": "computer",
                "display_width_px": display_width,
                "display_height_px": display_height,
                "display_number": display_number
            },
            {
                "type": "text_editor_20241022",
                "name": "str_replace_editor"
            },
            {
                "type": "bash_20241022",
                "name": "bash"
            }
        ],
        "endpoints": {
            "computer": "/computer-use/computer_20241022",
            "str_replace_editor": "/computer-use/text_editor_20241022",
            "bash": "/computer-use/bash_20241022"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting worker application");

    HttpServer::new(move || {
        App::new()
            .route("/computer-use/health", web::get().to(health_check))
            .service(
                web::scope("/computer-use")
                    // Use versioned endpoints but handle base tool names in responses
                    .route(
                        "/bash_20241022",
                        web::post().to(handlers::computer_use::handle_bash),
                    )
                    .route(
                        "/text_editor_20241022",
                        web::post().to(handlers::computer_use::handle_editor),
                    )
                    .route(
                        "/computer_20241022",
                        web::post().to(handlers::computer_use::handle_computer),
                    ),
            )
    })
    .bind("0.0.0.0:8081")?
    .workers(4)
    .run()
    .await
}
