use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse},
    web::{self, Data},
    Error,
};

use futures_util::future::LocalBoxFuture;
use std::sync::MutexGuard;
use uuid::Uuid;

use crate::db::calconnector::CalConnector;

use super::httpserver::AppState;
pub struct ApiKeyMiddleware<S> {
    pub service: S,
}

impl<S, B> ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    fn return_error(
        &self,
        error: String,
    ) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>> {
        Box::pin(async move { Err(actix_web::error::ErrorBadRequest(error)) })
    }
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
        let state: Data<AppState> = req.app_data::<web::Data<AppState>>().unwrap().clone();
        let cal_connector: &MutexGuard<CalConnector> = &state.cal_connector.lock().unwrap();
        let headers = req.headers().clone();
        let api_key = match headers.get("x-api-key") {
            Some(v) => match v.to_str() {
                Ok(v) => v,
                Err(error) => return self.return_error(error.to_string()),
            },
            None => return self.return_error("No API Key supplied in the request".to_string()),
        };

        let user_id_header_value = match headers.get("x-user-id") {
            Some(v) => match v.to_str() {
                Ok(v) => v,
                Err(error) => return self.return_error(error.to_string()),
            },
            None => return self.return_error("No UserId supplied in the request".to_string()),
        };

        let uuid = match Uuid::parse_str(user_id_header_value) {
            Ok(uuid) => uuid,
            Err(error) => return self.return_error(error.to_string()),
        };

        let user = match cal_connector.get_caluser(uuid) {
            Ok(user_option) => match user_option {
                Some(user) => user,
                None => return self.return_error("No User found with that Id".to_string()),
            },
            Err(error) => return self.return_error(error.to_string()),
        };

        if api_key != user.api_key {
            return self.return_error("API Key invalid".to_string());
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
