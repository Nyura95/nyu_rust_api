use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            repository,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError> {
        let mut cloned = user.clone();
        self.repository
            .create(&mut cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, user_id: i32) -> Result<User, CommonError> {
        self.repository
            .get(user_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, user_id: i32) -> Result<(), CommonError> {
        self.repository
            .delete(user_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
