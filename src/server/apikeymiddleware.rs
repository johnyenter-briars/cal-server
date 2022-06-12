use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error,
};

use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use crate::db::calconnector::CalConnector;
pub struct ApiKeyMiddleware<S> {
    pub service: S,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        let fut = self.service.call(req);
        Box::pin(async move {
            // let user_id_header_value = headers.get("x-user-id").ok_or_else(|| {
            //     actix_web::error::ErrorBadRequest("No UserId supplied in the request")
            // })?;

            // let api_key_header_value = headers.get("x-api-key").ok_or_else(|| {
            //     actix_web::error::ErrorBadRequest("No ApiKey supplied in the request")
            // })?;

            // let user_id = user_id_header_value.to_str().map_err(|_| actix_web::error::ErrorBadRequest(
            //         "Unable to convert UserId to string",
            //     ))?;

            // let api_key = api_key_header_value.to_str().map_err(|_| actix_web::error::ErrorBadRequest(
            //         "Unable to convert ApiKey to string",
            //     ))?;

            // let uuid = Uuid::parse_str(user_id).map_err(|_| actix_web::error::ErrorBadRequest("Unable to parse GUID"))?;

            // let user = CalConnector::get_caluser(uuid)?
            //     .ok_or_else(|| actix_web::error::ErrorBadRequest("No User found with that Id"))?;

            // if api_key != user.api_key {
            //     Err(actix_web::error::ErrorUnauthorized("API key is invalid"))
            // } else {
            //     let res = fut.await?;
            //     Ok(res)
            // }
            let res = fut.await?;
            Ok(res)
        })
    }
}
