use crate::{domain::models::service_context::ServiceContext, infrastructure::services::jwt::Claims};
use jsonwebtoken::{TokenData, errors::Error};

pub trait ServiceContextService: 'static + Sync + Send {
    fn get_service_context(&self) -> ServiceContext;
    fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, Error>;
    fn update(&self, service_context: ServiceContext) -> ServiceContext;
    fn is_maintenance_active(&self) -> bool;
}
