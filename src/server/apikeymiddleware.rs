use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error,
};

use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use crate::db::calconnector::CalConnector;
pub struct ApiKeyMiddleware<S> {
    pub service: S,
    pub key_value: String,
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
        //TODO: refac these to look like: req.headers().get("x-user-id").map(|id| id.to_str().unwrap_or(Box::pin...)).unwrap_or(Box::pin...)
        //// wrap the entire thing in an async block
        // Box::pin(async move {
        //     let user_id = req.headers()
        //         .get("x-user-id")
        //         .ok_or_else(|| actix_web::error::ErrorBadRequest("No UserId supplied in the request"))?;

        //     let user_id = user_id.to_str()
        //         ok_or_else(|| actix_web::error::ErrorBadRequest("UserId has non ASCII chars")?;

        //     // rest of the code...
        // })

        let user_id = match req.headers().get("x-user-id") {
            Some(id) => match id.to_str() {
                Ok(s) => s,
                Err(_) => {
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorBadRequest(
                            "UserId has non ASCII chars",
                        ))
                    })
                }
            },
            None => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorBadRequest(
                        "No UserId supplied in the request",
                    ))
                });
            }
        };

        let uuid = match Uuid::parse_str(&user_id) {
            Ok(id) => id,
            Err(_) => {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorBadRequest(
                        "GUID is improperly formatted",
                    ))
                });
            }
        };

        let user = match CalConnector::get_caluser(uuid) {
            Ok(u) => u,
            Err(e) => {
                return Box::pin(async move { Err(actix_web::error::ErrorBadRequest(e)) });
            }
        };

        match req.headers().get("x-api-key") {
            Some(k) => match k.to_str() {
                Ok(s) => {
                    if s == user.api_key {
                        let fut = self.service.call(req);

                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    } else {
                        Box::pin(async move {
                            Err(actix_web::error::ErrorUnauthorized("API key is invalid"))
                        })
                    }
                }
                Err(e) => Box::pin(async move { Err(actix_web::error::ErrorUnauthorized(e)) }),
            },
            None => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "API key not provided in request",
                ))
            }),
        }
    }
}
