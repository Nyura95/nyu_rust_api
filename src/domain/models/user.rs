use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct UserRole {
  pub id: i32,
  pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub username: String,
  pub password: String,
  pub role_id: i32,
  pub role: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct CreateUser {
  pub email: String,
  pub username: String,
  pub password: String,
  pub role_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Clone)]
pub struct UpdateUser {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub role_id: i32,
}

#[derive(Clone)]
pub struct LoggedInUser {
  pub email: String,
  pub username: String,
  pub role: String,
  pub token: String,
  pub refresh_token: String,
}

#[derive(Clone)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
  pub refresh_token: String,
}
