use crate::error::AppError;
use crate::models::user::{NewUser, UpdateUser};

use crate::db::DbPool;
use actix_web::{web, Error, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::services::user_service::UserService;
use crate::utils::jwt::{generate_token, validate_token as jwt_validate_token};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    if UserService::email_exists(&pool, &new_user.email)? {
        return Err(AppError::BadRequest("Email already in use".into()));
    }

    let user = UserService::create_user(&pool, new_user.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn login(
    pool: web::Data<DbPool>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    match UserService::login(&pool, &login_req.username, &login_req.password) {
        Ok(user) => match generate_token(user.id) {
            Ok(token) => Ok(HttpResponse::Ok().json(json!({ "token": token, "user": user }))),
            Err(e) => {
                log::error!("Token generation error: {:?}", e);
                Err(actix_web::error::ErrorInternalServerError(
                    "Failed to generate token",
                ))
            }
        },
        Err(e) => {
            log::error!("Login error: {:?}", e);
            match e {
                AppError::AuthenticationError => {
                    Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
                }
                _ => Err(actix_web::error::ErrorInternalServerError("Login failed")),
            }
        }
    }
}

pub async fn list_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match UserService::list_users(&pool) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            log::error!("Error listing users: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to list users",
            ))
        }
    }
}

use actix_web::HttpRequest;

pub async fn get_user(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    log::info!(
        "Received GET user request. User ID: {:?}, Headers: {:?}",
        user_id,
        req.headers()
    );
    match UserService::get_user(&pool, user_id.into_inner()) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Error getting user: {:?}", e);
            match e {
                AppError::NotFound => Err(actix_web::error::ErrorNotFound("User not found")),
                _ => Err(actix_web::error::ErrorInternalServerError(
                    "Failed to get user",
                )),
            }
        }
    }
}

pub async fn update_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
    user_data: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    match UserService::update_user(&pool, user_id.into_inner(), user_data.into_inner()) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Error updating user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update user",
            ))
        }
    }
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match UserService::delete_user(&pool, user_id.into_inner()) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            log::error!("Error deleting user: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete user",
            ))
        }
    }
}

pub async fn refresh_token(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                match UserService::refresh_token(&pool, token) {
                    Ok(new_token) => {
                        return Ok(HttpResponse::Ok().json(json!({ "token": new_token })));
                    }
                    Err(e) => {
                        log::error!("Failed to refresh token: {:?}", e);
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                    }
                }
            }
        }
    }
    log::warn!("Missing or invalid Authorization header");
    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
}

pub async fn validate_token(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    log::info!("Validate token function called");
    if let Some(auth_header) = req.headers().get("Authorization") {
        log::info!("Authorization header found: {:?}", auth_header);
        if let Ok(auth_str) = auth_header.to_str() {
            log::info!("Authorization string: {}", auth_str);
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                log::info!("Token extracted: {}", token);
                match jwt_validate_token(token) {
                    Ok((user_id, token_version)) => {
                        log::info!("Token validated successfully. User ID: {}, Version: {}", user_id, token_version);
                        // Check if the user exists in the database
                        match UserService::get_user(&pool, user_id) {
                            Ok(_) => {
                                return Ok(HttpResponse::Ok().json(json!({ "user_id": user_id, "token_version": token_version })))
                            }
                            Err(e) => {
                                log::error!("User not found: {:?}", e);
                                return Ok(HttpResponse::NotFound()
                                    .json(json!({ "error": "User not found" })));
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Token validation failed: {:?}", e);
                        return Ok(
                            HttpResponse::Unauthorized().json(json!({ "error": "Invalid token" }))
                        );
                    }
                }
            }
        }
    }
    log::warn!("Missing or invalid Authorization header");
    Ok(HttpResponse::Unauthorized()
        .json(json!({ "error": "Missing or invalid Authorization header" })))
}
