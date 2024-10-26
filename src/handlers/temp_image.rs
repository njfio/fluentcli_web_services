use actix_web::{get, web, HttpResponse};
use log::{debug, error};
use mime_guess::from_path;
use std::fs;
use std::path::PathBuf;

#[get("/temp-images/{filename}")]
pub async fn get_temp_image(filename: web::Path<String>) -> HttpResponse {
    let temp_dir = std::env::var("TEMP_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp/stability_images"));

    let file_path = temp_dir.join(filename.as_ref());
    debug!("Attempting to serve temp image: {:?}", file_path);

    match fs::read(&file_path) {
        Ok(content) => {
            let content_type = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();

            debug!("Serving temp image with content type: {}", content_type);
            HttpResponse::Ok().content_type(content_type).body(content)
        }
        Err(e) => {
            error!("Failed to read temp image file: {:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_temp_image);
}
