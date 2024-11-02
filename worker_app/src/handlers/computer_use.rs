use actix_web::{web, Error, HttpResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct BashRequest {
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorRequest {
    pub command: String,
    pub path: String,
    pub text: Option<String>,
    pub file_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerRequest {
    pub action: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub text: Option<String>,
}

/// Handle bash commands (bash_20241022)
pub async fn handle_bash(req: web::Json<BashRequest>) -> Result<HttpResponse, Error> {
    info!("Executing bash command: {}", req.command);

    let output = Command::new("bash")
        .arg("-c")
        .arg(&req.command)
        .output()
        .map_err(|e| {
            error!("Failed to execute command: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "name": "bash",
        "command": req.command,
        "output": {
            "stdout": stdout,
            "stderr": stderr,
            "status": output.status.code()
        }
    })))
}

/// Handle text editor operations (text_editor_20241022)
pub async fn handle_editor(req: web::Json<EditorRequest>) -> Result<HttpResponse, Error> {
    info!("Handling editor request: {:?}", req);

    match req.command.as_str() {
        "create" => {
            // Create parent directories if they don't exist
            if let Some(parent) = Path::new(&req.path).parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    error!("Failed to create directories: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;
            }

            // Write exact content without any modification
            let content = if let Some(file_text) = &req.file_text {
                file_text.as_str()
            } else if let Some(text) = &req.text {
                text.as_str()
            } else {
                ""
            };

            fs::write(&req.path, content).map_err(|e| {
                error!("Failed to write file: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "str_replace_editor",
                "command": "create",
                "path": req.path,
                "output": {
                    "success": true
                }
            })))
        }
        "read" => {
            // Read exact content without any modification
            let content = fs::read_to_string(&req.path).map_err(|e| {
                error!("Failed to read file: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "str_replace_editor",
                "command": "read",
                "path": req.path,
                "output": {
                    "content": content,
                    "success": true
                }
            })))
        }
        _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "name": "str_replace_editor",
            "command": req.command,
            "output": {
                "success": false,
                "error": format!("Unknown command: {}. Supported commands are 'create' and 'read'", req.command)
            }
        }))),
    }
}

/// Handle computer interactions (computer_20241022)
pub async fn handle_computer(req: web::Json<ComputerRequest>) -> Result<HttpResponse, Error> {
    info!("Handling computer request: {:?}", req);

    // Get display dimensions from environment
    let display_width: i32 = std::env::var("DISPLAY_WIDTH")
        .unwrap_or_else(|_| "1024".to_string())
        .parse()
        .unwrap_or(1024);
    let display_height: i32 = std::env::var("DISPLAY_HEIGHT")
        .unwrap_or_else(|_| "768".to_string())
        .parse()
        .unwrap_or(768);

    match req.action.as_str() {
        "click" => {
            if let (Some(x), Some(y)) = (req.x, req.y) {
                // Validate coordinates are within display bounds
                if x < 0 || x >= display_width || y < 0 || y >= display_height {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "name": "computer",
                        "action": "click",
                        "output": {
                            "success": false,
                            "error": format!("Coordinates ({}, {}) out of bounds. Display size is {}x{}", x, y, display_width, display_height)
                        }
                    })));
                }

                let output = Command::new("xdotool")
                    .args(["mousemove", &x.to_string(), &y.to_string(), "click", "1"])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "click",
                    "output": {
                        "coordinates": {
                            "x": x,
                            "y": y
                        },
                        "success": output.status.success()
                    }
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "name": "computer",
                    "action": "click",
                    "output": {
                        "success": false,
                        "error": "Missing coordinates for click action. Required: x and y"
                    }
                })))
            }
        }
        "type" => {
            if let Some(text) = &req.text {
                let output = Command::new("xdotool")
                    .args(["type", text])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "type",
                    "output": {
                        "text": text,
                        "success": output.status.success()
                    }
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "name": "computer",
                    "action": "type",
                    "output": {
                        "success": false,
                        "error": "Missing text parameter for type action"
                    }
                })))
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "name": "computer",
            "action": req.action,
            "output": {
                "success": false,
                "error": format!("Unknown action: {}. Supported actions are 'click' and 'type'", req.action)
            }
        }))),
    }
}
