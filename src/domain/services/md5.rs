pub trait Md5Service: 'static + Sync + Send {
  fn hash(&self, email: String, password: String) -> String;
  fn verify(&self, email: String, password: String, md5: String) -> bool;
}
