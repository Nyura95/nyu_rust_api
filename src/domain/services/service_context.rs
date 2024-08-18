use crate::{domain::models::service_context::ServiceContext, infrastructure::services::jwt::Claims};
use actix_web::HttpRequest;
use jsonwebtoken::{TokenData, errors::Error};

pub trait ServiceContextService: 'static + Sync + Send {
    fn get_service_context(&self) -> ServiceContext;
    fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, Error>;
    fn update(&self, service_context: ServiceContext) -> ServiceContext;
    fn is_maintenance_active(&self) -> bool;
    fn get_claims(&self, request: HttpRequest) -> Option<Claims>;
}
