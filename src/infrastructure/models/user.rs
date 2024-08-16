use chrono::{Utc, NaiveDateTime};
use diesel;
use diesel::prelude::*;
use crate::domain::models::user::{CreateUser, User};
use crate::infrastructure::schema::users;

#[derive(Queryable)]
pub struct UserDiesel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,   
    pub role_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Factory method for creating a new UserDiesel from a User
impl From<User> for UserDiesel {
    fn from(t: User) -> Self {
        UserDiesel {
            id: t.id,
            username: t.username,
            email: t.email,
            password: String::new(),
            role_id: t.role_id,
            created_at: t.created_at,
            updated_at: t.updated_at
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel {
  pub email: String,
  pub username: String,
  pub password: String,
  pub role_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

// Factory method for creating a new User from a UserDiesel
impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User {
          id: self.id,
          email: self.email,
          username: self.username,
          role_id: self.role_id,
          created_at: self.created_at,
          updated_at: self.updated_at,
        }
    }
}

impl From<CreateUser> for CreateUserDiesel {
    fn from(t: CreateUser) -> Self {
        CreateUserDiesel {
            email: t.email,
            password: t.password,
            username: t.username,
            role_id: t.role_id,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

impl Into<User> for CreateUserDiesel {
    fn into(self) -> User {
        User {
            id: 0,
            email: self.email,
            username: self.username,
            role_id: self.role_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}