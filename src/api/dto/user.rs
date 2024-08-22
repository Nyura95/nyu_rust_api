use crate::domain::models::user::{CreateUser, LoggedInUser, LoginUser, UpdateUser, User};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::domain::repositories::repository::ResultPaging;

#[derive(Deserialize, Serialize)]
pub struct LoggedUserDTO {
    pub email: String,
    pub username: String,
    pub role: String,
    pub token: String,
    pub refresh_token: String,
}

impl Into<LoggedUserDTO> for LoggedInUser {
    fn into(self) -> LoggedUserDTO {
        LoggedUserDTO {
            email: self.email,
            username: self.username,
            role: self.role,
            token: self.token,
            refresh_token: self.refresh_token,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
    pub refresh_token: String,
}

impl Into<LoginUser> for LoginUserDTO {
    fn into(self) -> LoginUser {
        LoginUser {
            email: self.email,
            password: self.password,
            refresh_token: self.refresh_token,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginDTO {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
    pub role_id: i32,
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            username: self.username,
            email: self.email,
            password: self.password,
            role_id: self.role_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
  pub id: i32,
  pub email: String,
  pub username: String,
  pub role: String,
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

#[derive(Deserialize, Serialize)]
pub struct UpdateUserDTO {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role_id: i32,
}

impl Into<UpdateUser> for UpdateUserDTO {
    fn into(self) -> UpdateUser {
        UpdateUser {
            id: self.id,
            username: self.username,
            password: self.password,
            role_id: self.role_id,
        }
    }
}

impl Into<UpdateUser> for User {
    fn into(self) -> UpdateUser {
        UpdateUser {
            id: self.id,
            username: self.username,
            password: self.password,
            role_id: self.role_id,
        }
    }
}
