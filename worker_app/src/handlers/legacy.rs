use actix_web::{web, Error, HttpResponse};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerToolRequest {
    pub action: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextEditorRequest {
    pub command: String,
    pub path: Option<String>,
    pub text: Option<String>,
    pub file_text: Option<String>,
    pub pattern: Option<String>,
    pub replacement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BashRequest {
    pub command: String,
}

pub async fn handle_computer_request(
    req: web::Json<ComputerToolRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received computer request: {:?}", req);
    match req.action.as_str() {
        "click" => {
            if let (Some(x), Some(y)) = (req.x, req.y) {
                info!("Executing click at coordinates ({}, {})", x, y);
                let output = Command::new("xdotool")
                    .args(["mousemove", &x.to_string(), &y.to_string(), "click", "1"])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    error!("xdotool failed: {}", error);
                    return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("xdotool failed: {}", error)
                    })));
                }
                info!("Click action successful");
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "action": "click",
                    "x": x,
                    "y": y
                })))
            } else {
                error!("Missing x or y coordinates for click action");
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing x or y coordinates for click action"
                })))
            }
        }
        "type" => {
            if let Some(text) = &req.text {
                info!("Executing type action with text: {}", text);
                let output = Command::new("xdotool")
                    .args(["type", text])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                if !output.status.success() {
                    let error = String::from_utf8_lossy(&output.stderr);
                    error!("xdotool failed: {}", error);
                    return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("xdotool failed: {}", error)
                    })));
                }
                info!("Type action successful");
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "action": "type",
                    "text": text
                })))
            } else {
                error!("Missing text for type action");
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing text for type action"
                })))
            }
        }
        _ => {
            error!("Unknown action: {}", req.action);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Unknown action: {}", req.action)
            })))
        }
    }
}

pub async fn handle_text_editor_request(
    req: web::Json<TextEditorRequest>,
) -> Result<HttpResponse, Error> {
    info!("Received text editor request: {:?}", req);
    match req.command.as_str() {
        "create" => {
            if let Some(path) = &req.path {
                info!("Creating file at path: {}", path);
                if let Some(parent) = Path::new(path).parent() {
                    info!("Creating parent directories: {:?}", parent);
                    fs::create_dir_all(parent).map_err(|e| {
                        error!("Failed to create directories: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;
                }

                let content = req
                    .file_text
                    .as_deref()
                    .or(req.text.as_deref())
                    .unwrap_or_default();
                info!("Writing content to file, content length: {}", content.len());
                fs::write(path, content).map_err(|e| {
                    error!("Failed to write file: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

                info!("File created successfully");
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "command": "create",
                    "path": path,
                    "text": content
                })))
            } else {
                error!("Missing path for create command");
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing path for create command"
                })))
            }
        }
        "replace" => {
            if let (Some(path), Some(pattern), Some(replacement)) =
                (&req.path, &req.pattern, &req.replacement)
            {
                info!("Replacing text in file: {}", path);
                let content = fs::read_to_string(path).map_err(|e| {
                    error!("Failed to read file: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

                let new_content = content.replace(pattern, replacement);
                info!("Writing updated content to file");
                fs::write(path, new_content).map_err(|e| {
                    error!("Failed to write file: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

                info!("File updated successfully");
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "command": "replace",
                    "path": path
                })))
            } else {
                error!("Missing path, pattern, or replacement for replace command");
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing path, pattern, or replacement for replace command"
                })))
            }
        }
        _ => {
            error!("Unknown command: {}", req.command);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Unknown command: {}", req.command)
            })))
        }
    }
}

pub async fn handle_bash_request(req: web::Json<BashRequest>) -> Result<HttpResponse, Error> {
    info!("Received bash request: {:?}", req);
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

    if output.status.success() {
        info!("Command executed successfully");
        debug!("Command stdout: {}", stdout);
        debug!("Command stderr: {}", stderr);
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "stdout": stdout,
            "stderr": stderr
        })))
    } else {
        error!("Command failed: {}", stderr);
        Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Command failed: {}", stderr)
        })))
    }
}
