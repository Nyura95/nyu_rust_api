use jsonwebtoken::TokenData;
use chrono::Duration;

use crate::{domain::error::CommonError, infrastructure::{models::user::UserRoleFormat, services::jwt::Claims}};

pub trait JwtService: 'static + Sync + Send {
  fn create_token(&self, user_id: i32, role: UserRoleFormat, expiration: Duration, is_refresh: bool) -> Result<String, CommonError>;
  fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, CommonError>;
}
