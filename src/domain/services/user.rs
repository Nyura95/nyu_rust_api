use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, LoginUser, LoggedInUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn login(&self, login_user: LoginUser) -> Result<LoggedInUser, CommonError>;
    async fn create(&self, user: CreateUser) -> Result<User, CommonError>;
    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, CommonError>;
    async fn get(&self, user_id: i32) -> Result<User, CommonError>;
    async fn delete(&self, user_id: i32) -> Result<(), CommonError>;
}
