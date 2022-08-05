use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::future::{ready, Ready};

use super::apikeymiddleware::ApiKeyMiddleware;
#[derive(Clone)]
pub struct ApiKey {
    pub key_value: String,
}

impl ApiKey {
    pub fn new(key_value: String) -> Self {
        ApiKey { key_value }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware { service }))
    }
}
