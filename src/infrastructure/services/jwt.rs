use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Error};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

use crate::domain::services::jwt::JwtService;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    exp: i64,
    refresh: bool,
}

pub struct JwtServiceImpl {
    secret: String,
}

impl JwtServiceImpl {
    pub fn new(secret: String) -> Self {
        JwtServiceImpl { secret }
    }
}

impl JwtService for JwtServiceImpl {
    fn create_token(&self, user_id: i32, expiration: Duration, is_refresh: bool) -> Result<String, Error> {
        let expiration_time = Utc::now() + expiration;
        
        let claims = Claims {
            sub: user_id,
            exp: expiration_time.timestamp(),
            refresh: is_refresh,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }

    fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
    }
}
