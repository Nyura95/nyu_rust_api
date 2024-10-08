use std::sync::Arc;
use std::env;
use dotenv::dotenv;

use crate::domain::constants::SECRET_JWT;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::todo::TodoService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::todo::TodoDieselRepository;
use crate::infrastructure::repositories::user::UserDieselRepository;
use crate::infrastructure::services::jwt::JwtServiceImpl;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::infrastructure::services::md5::Md5ServiceImpl;
use crate::services::todo::TodoServiceImpl;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>,
    pub service_context_service: Arc<dyn ServiceContextService>
}

impl Container {
    pub fn new() -> Self {
        dotenv().ok();
        let pool = Arc::new(db_pool());
        let md5_service = Arc::new(
            Md5ServiceImpl {}
        );
        
        let secret = env::var(SECRET_JWT)
            .expect(&*format!("{value} must be set", value = SECRET_JWT));
        let jwt_service = Arc::new(
            JwtServiceImpl::new(secret)
        );

        let todo_repository: Arc<dyn TodoRepository> = Arc::new(
            TodoDieselRepository::new(pool.clone())
        );
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserDieselRepository::new(pool.clone())
        );
        let todo_service = Arc::new(
            TodoServiceImpl { repository: todo_repository }
        );
        let user_service = Arc::new(
            UserServiceImpl { repository: user_repository, md5_service, jwt_service: jwt_service.clone()}
        );
        let service_context_service = Arc::new(
            ServiceContextServiceImpl::new(pool.clone(), jwt_service)
        );
        Container { user_service, todo_service, service_context_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
