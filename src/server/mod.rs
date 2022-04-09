use std::{fmt::Debug, rc::Rc};

use crate::routes::{
    caluserroutes::{create_caluser, get_caluser},
    eventroutes::{create_event, get_events},
    seriesroutes::{create_series, get_series},
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    middleware::Logger,
    App, Error, HttpServer,
};
use std::future::{ready, Ready};

use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Clone)]
pub struct ApiKey {
    key_value: String,
}

impl ApiKey {
    fn new(key_value: String) -> Self {
        ApiKey { key_value }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
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
        ready(Ok(ApiKeyMiddleware {
            service,
            key_value: self.key_value.clone(),
        }))
    }
}

pub struct ApiKeyMiddleware<S> {
    service: S,
    key_value: String,
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
        match req.headers().get("Api-Key") {
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
                Err(_) => todo!(),
            },
            None => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized(
                    "API key not provided in request",
                ))
            }),
        }
    }
}

pub async fn build_and_run_server(
    domain: String,
    port: u16,
    key_value: String,
) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(create_event)
            .service(get_events)
            .service(create_caluser)
            .service(get_caluser)
            .service(create_series)
            .service(get_series)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %U %{Content-Type}i"))
            .wrap(ApiKey::new(key_value.clone()))
    })
    .bind((domain, port))?
    .run()
    .await
}
