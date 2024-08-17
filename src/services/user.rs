use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::domain::services::md5::Md5Service;
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
    pub md5_service: Arc<dyn Md5Service>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>, md5_service: Arc<dyn Md5Service>) -> Self {
        UserServiceImpl {
            repository,
            md5_service,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError> {
        match self.repository.get_by_email(user.email.clone()).await {
            Ok(_) => {
                return Err(CommonError::entity_already_exist())
            },
            Err(_) => {},
        }

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
