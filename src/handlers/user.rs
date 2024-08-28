use crate::error::AppError;
use actix_web::{web, HttpResponse, Responder, Error};
use crate::db::DbPool;

use crate::services::user_service::UserService;
use crate::utils::jwt::generate_token;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    pool: web::Data<DbPool>,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    match UserService::login(&pool, &login_req.username, &login_req.password) {
        Ok(user) => {
            match generate_token(user.id) {
                Ok(token) => Ok(HttpResponse::Ok().json(LoginResponse { token })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
            }
        },
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}

pub async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("List users")
}

pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Get user")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Update user")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user")
}

pub async fn refresh_token() -> impl Responder {
    HttpResponse::Ok().body("Refresh token")
}