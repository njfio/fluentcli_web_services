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
    pub path: Option<String>,
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
        "success": output.status.success(),
        "stdout": stdout,
        "stderr": stderr
    })))
}

/// Handle text editor operations (text_editor_20241022)
pub async fn handle_editor(req: web::Json<EditorRequest>) -> Result<HttpResponse, Error> {
    info!("Handling editor request: {:?}", req);

    match req.command.as_str() {
        "create" => {
            if let Some(path) = &req.path {
                // Create parent directories if they don't exist
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        error!("Failed to create directories: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;
                }

                // Use file_text if available, fall back to text
                let content = req
                    .file_text
                    .as_deref()
                    .or(req.text.as_deref())
                    .unwrap_or_default();

                fs::write(path, content).map_err(|e| {
                    error!("Failed to write file: {}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": format!("File created: {}", path)
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "Missing path parameter"
                })))
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": format!("Unknown command: {}", req.command)
        }))),
    }
}

/// Handle computer interactions (computer_20241022)
pub async fn handle_computer(req: web::Json<ComputerRequest>) -> Result<HttpResponse, Error> {
    info!("Handling computer request: {:?}", req);

    match req.action.as_str() {
        "click" => {
            if let (Some(x), Some(y)) = (req.x, req.y) {
                let output = Command::new("xdotool")
                    .args(["mousemove", &x.to_string(), &y.to_string(), "click", "1"])
                    .output()
                    .map_err(|e| {
                        error!("Failed to execute xdotool: {}", e);
                        actix_web::error::ErrorInternalServerError(e)
                    })?;

                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "success": output.status.success(),
                    "message": format!("Clicked at ({}, {})", x, y)
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "Missing coordinates"
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
                    "success": output.status.success(),
                    "message": format!("Typed text: {}", text)
                })))
            } else {
                Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "Missing text parameter"
                })))
            }
        }
        _ => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": format!("Unknown action: {}", req.action)
        }))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_bash_handler() {
        let app = test::init_service(App::new().route("/bash", web::post().to(handle_bash))).await;

        let req = test::TestRequest::post()
            .uri("/bash")
            .set_json(BashRequest {
                command: "echo 'test'".to_string(),
            })
            .to_request();

        let resp: Value = test::call_and_read_body_json(&app, req).await;
        assert!(resp["success"].as_bool().unwrap());
        assert!(resp["stdout"].as_str().unwrap().contains("test"));
    }

    #[actix_web::test]
    async fn test_editor_handler() {
        let app =
            test::init_service(App::new().route("/editor", web::post().to(handle_editor))).await;

        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        let req = test::TestRequest::post()
            .uri("/editor")
            .set_json(EditorRequest {
                command: "create".to_string(),
                path: Some(test_file.to_str().unwrap().to_string()),
                text: Some("test content".to_string()),
                file_text: None,
            })
            .to_request();

        let resp: Value = test::call_and_read_body_json(&app, req).await;
        assert!(resp["success"].as_bool().unwrap());
        assert!(test_file.exists());
    }

    #[actix_web::test]
    async fn test_computer_handler() {
        let app =
            test::init_service(App::new().route("/computer", web::post().to(handle_computer)))
                .await;

        let req = test::TestRequest::post()
            .uri("/computer")
            .set_json(ComputerRequest {
                action: "type".to_string(),
                x: None,
                y: None,
                text: Some("test".to_string()),
            })
            .to_request();

        let resp: Value = test::call_and_read_body_json(&app, req).await;
        assert!(resp.get("success").is_some());
    }
}
