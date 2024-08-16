use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::models::user::{CreateUser, User};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::user::{CreateUserDiesel, UserDiesel};

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>
}

impl UserDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {

    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::users;
        let new_user_diesel: CreateUserDiesel = CreateUserDiesel::from(new_user.clone());
        let mut conn = self.pool.get().unwrap();
        let result: UserDiesel = run(move || diesel::insert_into(users).values(new_user_diesel)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>> {
        use crate::infrastructure::schema::users::dsl::users;
        let pool = self.pool.clone();
        let builder = users.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<UserDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect()
        })
    }

    async fn get(&self, user_id: i32) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::{id, users};
        let mut conn = self.pool.get().unwrap();
        run(move || users.filter(id.eq(user_id)).first::<UserDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> User { v.into() })
    }

    async fn delete(&self, user_id: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::users::dsl::{id, users};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(users).filter(id.eq(user_id))
            .execute(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
}