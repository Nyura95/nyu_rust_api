use crate::domain::models::user::{CreateUser, User};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::domain::repositories::repository::ResultPaging;

#[derive(Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role_id: i32,
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        let password = format!("{:x}", md5::compute(format!("{}:{}", self.username, self.password)));
        
        CreateUser {
            username: self.username,
            email: self.email,
            password: password,
            role_id: self.role_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
  id: i32,
  email: String,
  username: String,
  role: String,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            email: self.email,
            username: self.username,
            role: self.role,
        }
    }
}

impl Into<CreateUserDTO> for CreateUser {
    fn into(self) -> CreateUserDTO {
        CreateUserDTO {
          username: self.username,
          email: self.email,
          password: self.password,
          role_id: self.role_id,
        }
    }
}

impl Into<ResultPaging<UserDTO>> for ResultPaging<User> {
    fn into(self) -> ResultPaging<UserDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|user| user.into()).collect(),
        }
    }
}