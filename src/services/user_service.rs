use std::fmt::Debug;
use crate::db::DbPool;
use crate::error::AppError;
use crate::utils::auth::{hash_password, verify_password};
use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn create_user(
        pool: &DbPool,
        user: String,
        pass: String,
        password: String,
    ) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut pool.get()?;
        let hashed_password = hash_password(&password)?;

        let new_user = NewUser {
            username: user,
            email: pass,
            password_hash: hashed_password,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
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

    pub fn login(pool: &DbPool, username: &str, password: &str) -> Result<User, AppError> {
        let user = Self::get_user_by_username(pool, username.to_string())?;        
        if verify_password(password, &user.password_hash)? {
            Ok(user)
        } else {
            Err(AppError::AuthenticationError)
        }
    }
}
