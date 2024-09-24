use crate::db::DbPool;
use crate::error::AppError;
use crate::models::user::{NewUser, NewUserDB, UpdateUser, User};
use crate::utils::auth::{hash_password, verify_password};
use crate::utils::jwt;
use actix_web::HttpMessage;
use chrono::Utc;
use diesel::prelude::*;
use std::fmt::Debug;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn create_user(pool: &DbPool, new_user: NewUser) -> Result<User, AppError> {
        use crate::schema::users;
        let conn = &mut pool.get()?;

        let hashed_password = hash_password(&new_user.password)?;
        let new_user_db = NewUserDB {
            username: new_user.username,
            email: new_user.email,
            password_hash: hashed_password, // Use password_hash field
        };

        diesel::insert_into(users::table)
            .values(&new_user_db) // Use new_user_db for insertion
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn login(pool: &DbPool, username: &str, password: &str) -> Result<User, AppError> {
        let user = Self::get_user_by_username(pool, username.to_string())?;
        match verify_password(password, &user.password_hash) {
            Ok(is_valid) => {
                if is_valid {
                    Ok(user)
                } else {
                    Err(AppError::AuthenticationError)
                }
            }
            Err(e) => {
                log::error!("Password verification error: {:?}", e);
                Err(AppError::InternalServerError)
            }
        }
    }

    pub fn get_user_by_username(pool: &DbPool, username_: String) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut pool.get()?;

        users
            .filter(username.eq(username_))
            .first(conn)
            .map_err(|error| {
                if error == diesel::result::Error::NotFound {
                    AppError::NotFound
                } else {
                    AppError::DatabaseError(error)
                }
            })
    }

    pub fn get_user(pool: &DbPool, user_id: Uuid) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;
        let conn = &mut pool.get()?;
        log::info!("Received GET user request. User ID: {:?},", user_id);
        users
            .find(user_id)
            .first(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn update_user(
        pool: &DbPool,
        user_id: Uuid,
        update_data: UpdateUser,
    ) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;
        let conn = &mut pool.get()?;
        diesel::update(users.find(user_id))
            .set(&update_data)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    pub fn delete_user(pool: &DbPool, user_id: Uuid) -> Result<(), AppError> {
        use crate::schema::users::dsl::*;
        let conn = &mut pool.get()?;
        diesel::delete(users.find(user_id))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }

    pub fn email_exists(pool: &DbPool, email: &str) -> Result<bool, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut pool.get()?;
        let count = users
            .filter(email.eq(email))
            .count()
            .get_result::<i64>(conn)?;

        Ok(count > 0)
    }

    pub fn list_users(pool: &DbPool) -> Result<Vec<User>, AppError> {
        use crate::schema::users::dsl::*; // Add this line to import users
        let conn = &mut pool.get()?;
        users.load::<User>(conn).map_err(AppError::DatabaseError)
    }

    pub fn refresh_token(pool: &DbPool, old_token: &str) -> Result<String, AppError> {
        let (user_id, _) = jwt::validate_token(old_token)?;
        jwt::generate_token(user_id)
    }
}
