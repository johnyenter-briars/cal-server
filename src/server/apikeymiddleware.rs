use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    Error,
};

use futures_util::future::LocalBoxFuture;
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
        match req.headers().get("x-api-key") {
            Some(k) => match k.to_str() {
                Ok(s) => {
                    if s == self.key_value {
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
                Err(e) => Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(
                        e,
                    ))
                }),
            },
            None => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "API key not provided in request",
                ))
            }),
        }
    }
}
