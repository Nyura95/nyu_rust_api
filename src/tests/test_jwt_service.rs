#[cfg(test)]
mod test_jwt_service{
  use chrono::{Duration, Utc};
  use actix_clean_architecture::{domain::services::jwt::JwtService, infrastructure::services::jwt::JwtServiceImpl};

  #[test]
    fn test_create_and_validate_token() {
        let secret = "my_secret_key".to_string();
        let jwt_service = JwtServiceImpl::new(secret.clone());

        let user_id = 1;
        let role_id = 2;
        let expiration = Duration::minutes(30);
        let is_refresh = false;

        // Créer un token
        let token = jwt_service.create_token(user_id, role_id, expiration, is_refresh);
        assert!(token.is_ok(), "Failed to create token: {:?}", token.err());

        let token = token.unwrap();

        // Valider le token
        let token_data = jwt_service.validate_token(&token);
        assert!(token_data.is_ok(), "Failed to validate token: {:?}", token_data.err());

        let claims = token_data.unwrap().claims;
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role_id, role_id);
        assert_eq!(claims.refresh, is_refresh);
        assert!(claims.exp > Utc::now().timestamp());
    }

    #[test]
    fn test_validate_expired_token() {
        let secret = "my_secret_key".to_string();
        let jwt_service = JwtServiceImpl::new(secret.clone());

        let user_id = 1;
        let role_id = 2;
        let expiration = Duration::seconds(-30); // Expiration dans le passé
        let is_refresh = false;

        // Créer un token expiré
        let token = jwt_service.create_token(user_id, role_id, expiration, is_refresh);
        assert!(token.is_ok(), "Failed to create token: {:?}", token.err());

        let token = token.unwrap();

        // Valider le token expiré
        let token_data = jwt_service.validate_token(&token);
        assert!(!token_data.is_err(), "Token should be expired");

        if let Err(err) = token_data {
          panic!("Unexpected error kind: {:?}", err)
        }
    }

    #[test]
    fn test_validate_token_with_invalid_secret() {
        let secret = "my_secret_key".to_string();
        let jwt_service = JwtServiceImpl::new(secret.clone());

        let user_id = 1;
        let role_id = 2;
        let expiration = Duration::minutes(30);
        let is_refresh = false;

        // Créer un token valide
        let token = jwt_service.create_token(user_id, role_id, expiration, is_refresh);
        assert!(token.is_ok(), "Failed to create token: {:?}", token.err());

        let token = token.unwrap();

        // Essayer de valider le token avec une mauvaise clé secrète
        let invalid_secret_service = JwtServiceImpl::new("wrong_secret_key".to_string());
        let token_data = invalid_secret_service.validate_token(&token);
        assert_eq!(token_data.is_err(), true, "Token should be invalid due to wrong secret");
    }
}