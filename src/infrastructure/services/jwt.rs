use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

use crate::{domain::{error::CommonError, services::jwt::JwtService}, infrastructure::models::user::UserRoleFormat};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub role_id: i32,
    pub exp: i64,
    pub refresh: bool,
}

#[derive(Clone)]
pub struct JwtServiceImpl {
    secret: String,
}

impl JwtServiceImpl {
    pub fn new(secret: String) -> Self {
        JwtServiceImpl { secret }
    }
}

impl JwtService for JwtServiceImpl {
    fn create_token(&self, user_id: i32, role: UserRoleFormat, expiration: Duration, is_refresh: bool) -> Result<String, CommonError> {
        let expiration_time = Utc::now() + expiration;
        
        let claims = Claims {
            sub: user_id,
            role_id: role.into(),
            exp: expiration_time.timestamp(),
            refresh: is_refresh,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
            .map_err(|e| e.into())
    }

    fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, CommonError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        ).map_err(|e| e.into())
    }
}
