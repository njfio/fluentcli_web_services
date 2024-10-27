use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;

mod computer_use;
use computer_use::{
    BashRequest, ComputerToolConfig, ComputerToolRequest, ComputerUseService, TextEditorRequest,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub output: String,
    pub error: Option<String>,
    pub exit_code: i32,
}

// Original command execution endpoint
async fn execute_command(command_request: web::Json<CommandRequest>) -> impl Responder {
    println!("Received command request: {:?}", command_request);

    let mut command = Command::new("fluent");
    command.arg(&command_request.command);
    command.args(&command_request.args);

    let output = command.output();

    println!("Command executed, output: {:?}", output);

    match output {
        Ok(output) => {
            let result = CommandResult {
                output: String::from_utf8_lossy(&output.stdout).to_string(),
                error: if output.status.success() {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                },
                exit_code: output.status.code().unwrap_or(-1),
            };
            println!("Sending response: {:?}", result);
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            println!("Error executing command: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to execute command: {}", e))
        }
    }
}

// Computer use endpoints
async fn handle_computer_action(
    computer_service: web::Data<Arc<ComputerUseService>>,
    request: web::Json<ComputerToolRequest>,
) -> impl Responder {
    let result = computer_service
        .handle_computer_request(request.into_inner())
        .await;
    HttpResponse::Ok().json(result)
}

async fn handle_text_editor(
    computer_service: web::Data<Arc<ComputerUseService>>,
    request: web::Json<TextEditorRequest>,
) -> impl Responder {
    let result = computer_service
        .handle_text_editor_request(request.into_inner())
        .await;
    HttpResponse::Ok().json(result)
}

async fn handle_bash(
    computer_service: web::Data<Arc<ComputerUseService>>,
    request: web::Json<BashRequest>,
) -> impl Responder {
    let result = computer_service
        .handle_bash_request(request.into_inner())
        .await;
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Initialize computer use service with default config
    let computer_service = Arc::new(ComputerUseService::new(ComputerToolConfig {
        display_width_px: 1024,
        display_height_px: 768,
        display_number: 1,
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(computer_service.clone()))
            // Original command endpoint
            .service(web::resource("/execute").route(web::post().to(execute_command)))
            // Computer use endpoints
            .service(
                web::scope("/computer-use")
                    .route("/computer", web::post().to(handle_computer_action))
                    .route("/text-editor", web::post().to(handle_text_editor))
                    .route("/bash", web::post().to(handle_bash)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
