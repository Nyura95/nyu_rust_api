use md5;

use crate::domain::services::md5::Md5Service;

#[derive(Clone)]
pub struct Md5ServiceImpl {
}

impl Md5Service for Md5ServiceImpl {
  fn hash(&self, email: String, password: String) -> String {
    return format!("{:x}", md5::compute(format!("{}:{}", email, password)));
  }

  fn verify(&self, email: String, password: String, md5: String) -> bool {
    return self.hash(email, password) == md5
  }
}