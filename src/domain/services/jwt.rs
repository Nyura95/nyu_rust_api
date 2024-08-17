use jsonwebtoken::{TokenData, errors::Error};
use chrono::Duration;

use crate::infrastructure::services::jwt::Claims;

pub trait JwtService: 'static + Sync + Send {
  fn create_token(&self, user_id: i32, expiration: Duration, is_refresh: bool) -> Result<String, Error>;
  fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, Error>;
}
