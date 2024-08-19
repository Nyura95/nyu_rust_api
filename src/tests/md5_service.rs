#[cfg(test)]
mod test_md5_service{
    use actix_clean_architecture::{domain::services::md5::Md5Service, infrastructure::services::md5::Md5ServiceImpl};

  #[test]
  fn test_hash_generation() {
      let md5_service = Md5ServiceImpl {};

      let email = "test@example.com".to_string();
      let password = "password123".to_string();

      let hash = md5_service.hash(email.clone(), password.clone());
      
      let expected_hash = format!("{:x}", md5::compute(format!("{}:{}", email, password)));

      assert_eq!(hash, expected_hash, "MD5 hash mismatch");
  }

  #[test]
  fn test_verify_valid_hash() {
      let md5_service = Md5ServiceImpl {};

      let email = "test@example.com".to_string();
      let password = "password123".to_string();

      let hash = md5_service.hash(email.clone(), password.clone());

      let is_valid = md5_service.verify(email, password, hash);

      assert!(is_valid, "Hash verification should be valid");
  }

  #[test]
  fn test_verify_invalid_hash() {
      let md5_service = Md5ServiceImpl {};

      let email = "test@example.com".to_string();
      let password = "password123".to_string();

      let hash = md5_service.hash(email.clone(), password.clone());

      let wrong_password = "wrongpassword".to_string();
      let is_valid = md5_service.verify(email, wrong_password, hash);

      assert!(!is_valid, "Hash verification should fail with wrong password");
  }

  #[test]
  fn test_verify_with_wrong_hash() {
      let md5_service = Md5ServiceImpl {};

      let email = "test@example.com".to_string();
      let password = "password123".to_string();

      md5_service.hash(email.clone(), password.clone());

      let wrong_hash = format!("{:x}", md5::compute("wrong_hash"));
      let is_valid = md5_service.verify(email, password, wrong_hash);

      assert!(!is_valid, "Hash verification should fail with wrong hash");
  }
}