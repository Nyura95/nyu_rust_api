use md5;

use crate::domain::services::md5::Md5Service;

#[derive(Clone)]
pub struct Md5ServiceImpl {
}

impl Md5Service for Md5ServiceImpl {
  fn hash(&self, username: String, password: String) -> String {
    return format!("{:x}", md5::compute(format!("{}:{}", username, password)));
  }

  fn verify(&self, username: String, password: String, md5: String) -> bool {
    return self.hash(username, password) == md5
  }
}