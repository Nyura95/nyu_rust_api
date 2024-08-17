pub trait Md5Service: 'static + Sync + Send {
  fn hash(&self, username: String, password: String) -> String;
  fn verify(&self, username: String, password: String, md5: String) -> bool;
}
