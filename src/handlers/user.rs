use crate::models::user::{NewUser, UpdateUser};
use crate::error::AppError;

use actix_web::{web, HttpResponse, Responder, Error};
use crate::db::DbPool;
use serde_json::json; 
use uuid::Uuid;





use crate::services::user_service::UserService;
use crate::utils::jwt::generate_token;
use serde::{Deserialize};

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
        Ok(user) => {
            match generate_token(user.id) {
                Ok(token) => Ok(HttpResponse::Ok().json(json!({ "token": token, "user": user }))),
                Err(e) => {
                    log::error!("Token generation error: {:?}", e);
                    Err(actix_web::error::ErrorInternalServerError("Failed to generate token"))
                }
            }
        },
        Err(e) => {
            log::error!("Login error: {:?}", e);
            match e {
                AppError::AuthenticationError => Err(actix_web::error::ErrorUnauthorized("Invalid credentials")),
                _ => Err(actix_web::error::ErrorInternalServerError("Login failed"))
            }
        }
    }
}

pub async fn list_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match UserService::list_users(&pool) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            log::error!("Error listing users: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Failed to list users"))
        }
    }
}

use actix_web::HttpRequest;

pub async fn get_user(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    log::info!("Received GET user request. User ID: {:?}, Headers: {:?}", user_id, req.headers());
    match UserService::get_user(&pool, user_id.into_inner()) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Error getting user: {:?}", e);
            match e {
                AppError::NotFound => Err(actix_web::error::ErrorNotFound("User not found")),
                _ => Err(actix_web::error::ErrorInternalServerError("Failed to get user"))
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
            Err(actix_web::error::ErrorInternalServerError("Failed to update user"))
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
            Err(actix_web::error::ErrorInternalServerError("Failed to delete user"))
        }
    }
}

pub async fn refresh_token(pool: web::Data<DbPool>, token: web::Json<String>) -> impl Responder {
    match UserService::refresh_token(&pool, &token.into_inner()) {
        Ok(new_token) => HttpResponse::Ok().json(serde_json::json!({ "token": new_token })),
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Invalid token" })),
    }
}

