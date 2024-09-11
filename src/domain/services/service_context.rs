use crate::{domain::{error::CommonError, models::service_context::ServiceContext}, infrastructure::services::jwt::Claims};
use actix_web::HttpRequest;
use jsonwebtoken::TokenData;

pub trait ServiceContextService: 'static + Sync + Send {
    fn get_service_context(&self) -> ServiceContext;
    fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, CommonError>;
    fn update(&self, service_context: ServiceContext) -> ServiceContext;
    fn is_maintenance_active(&self) -> bool;
    fn get_claims(&self, request: HttpRequest) -> Option<Claims>;
}
