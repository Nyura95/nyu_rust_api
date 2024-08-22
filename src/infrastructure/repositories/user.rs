use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::user::{UserQueryParams, UserRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::user::{CreateUserDiesel, UpdateUserDiesel, UserDiesel, UserRoleDiesel};

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>,
}

impl UserDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {

    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users;
        let new_user_diesel: CreateUserDiesel = CreateUserDiesel::from(new_user.clone());
        let mut conn = self.pool.get().unwrap();
        let result: i32 = run(move || diesel::insert_into(users::table).values(new_user_diesel).returning(users::id)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        return self.get(result).await
    }

    async fn update(&self, update_user: &UpdateUser) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users;
        let update_user_diesel: UpdateUserDiesel = UpdateUserDiesel::from(update_user.clone());
        let mut conn = self.pool.get().unwrap();

        let result: i32 = run(move || diesel::update(users::table.filter(users::id.eq(update_user_diesel.id))).set(&update_user_diesel).returning(users::id)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        return self.get(result).await
    }

    async fn list(&self, params: UserQueryParams) -> RepositoryResult<ResultPaging<User>> {
        use crate::infrastructure::schema::users;
        use crate::infrastructure::schema::user_role;
        let pool = self.pool.clone();
        let builder = users::table.inner_join(user_role::table).limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<(UserDiesel, UserRoleDiesel)>(&mut conn)
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
        use crate::infrastructure::schema::user_role;
        let mut conn = self.pool.get().unwrap();
        run(move || users.filter(id.eq(user_id)).inner_join(user_role::table).first::<(UserDiesel, UserRoleDiesel)>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> User { v.into() })
    }

    async fn get_by_email(&self, _email: String) -> RepositoryResult<User> {
        use crate::infrastructure::schema::users::dsl::{email, users};
        use crate::infrastructure::schema::user_role;
        let mut conn = self.pool.get().unwrap();
        run(move || users.filter(email.eq(_email)).inner_join(user_role::table).first::<(UserDiesel, UserRoleDiesel)>(&mut conn))
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