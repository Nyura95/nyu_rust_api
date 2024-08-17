use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::repositories::repository::{QueryParams, ResultPaging, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::domain::models::user::{User, CreateUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub username: Option<String>,
}

impl QueryParams for UserQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User>;
    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>>;
    async fn get(&self, user_id: i32) -> RepositoryResult<User>;
    async fn get_by_email(&self, email: String) -> RepositoryResult<User>;
    async fn delete(&self, user_id: i32) -> RepositoryResult<()>;
}
