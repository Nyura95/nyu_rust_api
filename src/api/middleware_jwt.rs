use std::future::{ready, Ready};
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, web,
    HttpMessage, // Ajout de cet import pour les extensions
};
use futures_util::future::LocalBoxFuture;
use crate::{domain::services::service_context::ServiceContextService, infrastructure::models::user::UserRoleFormat};

pub struct ServiceJwtCheck {
    min_role_id: UserRoleFormat,
}

impl ServiceJwtCheck {
    pub fn new(min_role_id: UserRoleFormat) -> Self {
      Self { min_role_id }
    }
  }

impl<S, B> Transform<S, ServiceRequest> for ServiceJwtCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ServiceJwtCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServiceJwtCheckMiddleware { 
            service,
            min_role_id: self.min_role_id.clone(),
        }))
    }
}

pub struct ServiceJwtCheckMiddleware<S> {
    service: S,
    min_role_id: UserRoleFormat,
}

impl<S, B> Service<ServiceRequest> for ServiceJwtCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let service_context_service =
            request.app_data::<web::Data<dyn ServiceContextService>>().unwrap();

        let mut authorize = false;
        if let Some(auth_header) = request.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match service_context_service.verify_token(token) {
                        Ok(token_data) => {
                            request.extensions_mut().insert(token_data.claims.clone());
                            if token_data.claims.role_id >= self.min_role_id.clone().into() {
                                authorize = true;
                            }
                        },
                        Err(_) => {},
                    }
                }
            }
        }

        if !authorize {
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Unauthorized().finish().map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);
        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}