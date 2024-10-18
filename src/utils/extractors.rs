use crate::utils::jwt::validate_token;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use uuid::Uuid;

pub struct AuthenticatedUser(pub Uuid);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        match auth_header {
            Some(auth_value) => {
                let auth_str = auth_value.to_str().unwrap_or("");
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Remove "Bearer " prefix
                    match validate_token(token) {
                        Ok((user_id, _)) => ready(Ok(AuthenticatedUser(user_id))),
                        Err(_) => ready(Err(Error::from(actix_web::error::ErrorUnauthorized(
                            "Invalid token",
                        )))),
                    }
                } else {
                    ready(Err(Error::from(actix_web::error::ErrorUnauthorized(
                        "Invalid authorization header",
                    ))))
                }
            }
            None => ready(Err(Error::from(actix_web::error::ErrorUnauthorized(
                "Missing authorization header",
            )))),
        }
    }
}
