use std::sync::Arc;

use async_trait::async_trait;
use chrono::Duration;

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, LoggedInUser, LoginUser, UpdateUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::domain::services::jwt::JwtService;
use crate::domain::services::md5::Md5Service;
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
    pub md5_service: Arc<dyn Md5Service>,
    pub jwt_service: Arc<dyn JwtService>, 
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>, md5_service: Arc<dyn Md5Service>, jwt_service: Arc<dyn JwtService>) -> Self {
        UserServiceImpl {
            repository,
            md5_service,
            jwt_service,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {

    async fn login(&self, login_user: LoginUser) -> Result<LoggedInUser, CommonError> {
        let find_user = self.repository.get_by_email(login_user.email.clone()).await.map_err(|e| -> CommonError { e.into() })?;

        if !self.md5_service.verify(login_user.email, login_user.password, find_user.password) {
            return Err(CommonError::bad_connection())
        }

        let token = self.jwt_service.create_token(find_user.id, find_user.role_id, Duration::hours(1), false).map_err(|e| -> CommonError { e.into() })?;
        let refresh_token = self.jwt_service.create_token(find_user.id, find_user.role_id, Duration::hours(1), true).map_err(|e| -> CommonError { e.into() })?;

        return Ok(LoggedInUser{
            email: find_user.email,
            username: find_user.username,
            role: find_user.role,
            token: token,
            refresh_token: refresh_token,
        })
    }

    async fn create(&self, user: CreateUser) -> Result<User, CommonError> {
        match self.repository.get_by_email(user.email.clone()).await {
            Ok(_) => {
                return Err(CommonError::entity_already_exist())
            },
            Err(_) => {},
        }

        let mut cloned = user.clone();
        cloned.password = self.md5_service.hash(user.email, user.password);

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

    async fn update(&self, update_user: UpdateUser) -> Result<User, CommonError> {
        self.repository
            .update(&update_user)
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
