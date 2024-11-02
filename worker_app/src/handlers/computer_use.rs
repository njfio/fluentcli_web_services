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
    pub key: Option<String>,
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
            if let Some(parent) = Path::new(&req.path).parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    error!("Failed to create directories: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;
            }

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

/// Take a screenshot of the Xvfb display
fn take_screenshot() -> Result<String, std::io::Error> {
    fs::create_dir_all("/tmp/screenshots")?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let xwd_file = format!("/tmp/screenshots/screen_{}.xwd", timestamp);
    let png_file = format!("/tmp/screenshots/screen_{}.png", timestamp);

    let xwd_output = Command::new("xwd")
        .args(["-root", "-silent", "-display", ":99"])
        .output()?;

    if !xwd_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to capture screenshot: {}",
                String::from_utf8_lossy(&xwd_output.stderr)
            ),
        ));
    }

    fs::write(&xwd_file, &xwd_output.stdout)?;

    let convert_output = Command::new("convert")
        .args([&xwd_file, &png_file])
        .output()?;

    if !convert_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to convert screenshot: {}",
                String::from_utf8_lossy(&convert_output.stderr)
            ),
        ));
    }

    let image_data = fs::read(&png_file)?;

    fs::remove_file(&xwd_file)?;
    fs::remove_file(&png_file)?;

    Ok(format!(
        "data:image/png;base64,{}",
        base64::encode(&image_data)
    ))
}

/// Get current cursor position
fn get_cursor_position() -> Result<(i32, i32), std::io::Error> {
    let output = Command::new("xdotool")
        .args(["getmouselocation"])
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get cursor position",
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.split_whitespace().collect();

    let x = parts
        .iter()
        .find(|p| p.starts_with("x:"))
        .and_then(|p| p[2..].parse::<i32>().ok())
        .unwrap_or(0);

    let y = parts
        .iter()
        .find(|p| p.starts_with("y:"))
        .and_then(|p| p[2..].parse::<i32>().ok())
        .unwrap_or(0);

    Ok((x, y))
}

/// Handle computer interactions (computer_20241022)
pub async fn handle_computer(req: web::Json<ComputerRequest>) -> Result<HttpResponse, Error> {
    info!("Handling computer request: {:?}", req);

    let display_width: i32 = std::env::var("DISPLAY_WIDTH")
        .unwrap_or_else(|_| "1024".to_string())
        .parse()
        .unwrap_or(1024);
    let display_height: i32 = std::env::var("DISPLAY_HEIGHT")
        .unwrap_or_else(|_| "768".to_string())
        .parse()
        .unwrap_or(768);

    match req.action.as_str() {
        "mouse_move" => {
            if let (Some(x), Some(y)) = (req.x, req.y) {
                if x < 0 || x >= display_width || y < 0 || y >= display_height {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "name": "computer",
                        "action": "mouse_move",
                        "output": {
                            "success": false,
                            "error": format!("Coordinates ({}, {}) out of bounds. Display size is {}x{}", x, y, display_width, display_height)
                        }
                    })));
                }

                let output = Command::new("xdotool")
                    .args(["mousemove", &x.to_string(), &y.to_string()])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "mouse_move",
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
                    "action": "mouse_move",
                    "output": {
                        "success": false,
                        "error": "Missing coordinates for mouse_move action. Required: x and y"
                    }
                })))
            }
        }
        "left_click" => {
            let output = Command::new("xdotool")
                .args(["click", "1"])
                .output()
                .map_err(|e| {
                    error!("Failed to execute xdotool: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "computer",
                "action": "left_click",
                "output": {
                    "success": output.status.success()
                }
            })))
        }
        "right_click" => {
            let output = Command::new("xdotool")
                .args(["click", "3"])
                .output()
                .map_err(|e| {
                    error!("Failed to execute xdotool: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "computer",
                "action": "right_click",
                "output": {
                    "success": output.status.success()
                }
            })))
        }
        "middle_click" => {
            let output = Command::new("xdotool")
                .args(["click", "2"])
                .output()
                .map_err(|e| {
                    error!("Failed to execute xdotool: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "computer",
                "action": "middle_click",
                "output": {
                    "success": output.status.success()
                }
            })))
        }
        "double_click" => {
            let output = Command::new("xdotool")
                .args(["click", "--repeat", "2", "1"])
                .output()
                .map_err(|e| {
                    error!("Failed to execute xdotool: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "name": "computer",
                "action": "double_click",
                "output": {
                    "success": output.status.success()
                }
            })))
        }
        "left_click_drag" => {
            if let (Some(x), Some(y)) = (req.x, req.y) {
                if x < 0 || x >= display_width || y < 0 || y >= display_height {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "name": "computer",
                        "action": "left_click_drag",
                        "output": {
                            "success": false,
                            "error": format!("Coordinates ({}, {}) out of bounds. Display size is {}x{}", x, y, display_width, display_height)
                        }
                    })));
                }

                let output = Command::new("xdotool")
                    .args(["mousedown", "1", "mousemove", &x.to_string(), &y.to_string(), "mouseup", "1"])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "left_click_drag",
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
                    "action": "left_click_drag",
                    "output": {
                        "success": false,
                        "error": "Missing coordinates for left_click_drag action. Required: x and y"
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
        "key" => {
            if let Some(key) = &req.key {
                let output = Command::new("xdotool")
                    .args(["key", key])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "key",
                    "output": {
                        "key": key,
                        "success": output.status.success()
                    }
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "name": "computer",
                    "action": "key",
                    "output": {
                        "success": false,
                        "error": "Missing key parameter for key action"
                    }
                })))
            }
        }
        "screenshot" => {
            match take_screenshot() {
                Ok(image) => Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "screenshot",
                    "output": {
                        "success": true,
                        "image": image
                    }
                }))),
                Err(e) => {
                    error!("Screenshot error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "name": "computer",
                        "action": "screenshot",
                        "output": {
                            "success": false,
                            "error": format!("Failed to take screenshot: {}", e)
                        }
                    })))
                }
            }
        }
        "cursor_position" => {
            match get_cursor_position() {
                Ok((x, y)) => Ok(HttpResponse::Ok().json(serde_json::json!({
                    "name": "computer",
                    "action": "cursor_position",
                    "output": {
                        "success": true,
                        "coordinates": {
                            "x": x,
                            "y": y
                        }
                    }
                }))),
                Err(e) => {
                    error!("Cursor position error: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "name": "computer",
                        "action": "cursor_position",
                        "output": {
                            "success": false,
                            "error": format!("Failed to get cursor position: {}", e)
                        }
                    })))
                }
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "name": "computer",
            "action": req.action,
            "output": {
                "success": false,
                "error": format!("Unknown action: {}. Supported actions are 'key', 'type', 'mouse_move', 'left_click', 'left_click_drag', 'right_click', 'middle_click', 'double_click', 'screenshot', 'cursor_position'", req.action)
            }
        }))),
    }
}
