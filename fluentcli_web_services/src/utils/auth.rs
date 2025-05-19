use crate::error::AppError;
use crate::utils::jwt::validate_token;
use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use bcrypt::{hash, verify, DEFAULT_COST};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|_| AppError::InternalServerError)
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                match validate_token(token) {
                    Ok((user_id, _)) => {
                        req.extensions_mut().insert(user_id);
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                    Err(e) => {
                        log::error!("Token validation failed: {:?}", e);
                        return Box::pin(async {
                            Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                        });
                    }
                }
            } else {
                log::warn!("Malformed Authorization header: {}", auth_header);
            }
        } else {
            log::warn!("Missing Authorization header");
        }

        Box::pin(async {
            Err(actix_web::error::ErrorUnauthorized(
                "Authorization header missing or malformed",
            ))
        })
    }
}
