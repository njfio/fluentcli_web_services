use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct StopRequest {
    pub run_id: String,
}

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

async fn stop_command(_req: web::Json<StopRequest>) -> impl Responder {
    println!("Received stop request: {:?}", _req);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/execute").route(web::post().to(execute_command)))
            .service(web::resource("/stop").route(web::post().to(stop_command)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
