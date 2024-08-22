#[cfg(test)]
mod test_todo_controllers{
    use std::env;
    use std::sync::Arc;
    use actix_clean_architecture::api::dto::user::UserDTO;
    use actix_clean_architecture::domain::constants::SECRET_JWT;
    use actix_clean_architecture::domain::services::jwt::JwtService;
    use actix_clean_architecture::infrastructure::models::user::UserRoleFormat;
    use actix_clean_architecture::infrastructure::services::jwt::JwtServiceImpl;
    use actix_web::http::header::HeaderValue;
    use actix_web::test;
    use serde_json;
    use chrono::Duration;
    use actix_clean_architecture::infrastructure::databases::postgresql::db_pool;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use serde_json::json;
    use actix_clean_architecture::{container::Container, create_app::create_app};
    use actix_clean_architecture::domain::repositories::repository::ResultPaging;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    #[actix_web::test]
    async fn test() {
        {
            let pool = Arc::new(db_pool());
            pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();
        }

        let container = Arc::new(Container::new());

        let app = test::init_service(create_app(container)).await;
        let request_body = json!({
            "email": "test&test.com",
            "username": "username",
            "password": "password",
            "role_id": 1
        });

        // create token Player
        let secret = env::var(SECRET_JWT)
            .expect(&*format!("{value} must be set", value = SECRET_JWT));
        let jwt_service = JwtServiceImpl::new(secret);
        let jwt = jwt_service.create_token(1, UserRoleFormat::Administrator.into(), Duration::hours(1), false).unwrap();
        let token = format!("Bearer {}", jwt);

        // Check if empty table (with default admin)
        let req = test::TestRequest::get().uri("/users").append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let users: ResultPaging<UserDTO> = test::read_body_json(resp).await;
        assert_eq!(users.items.len(), 1);

        // Creation test
        let resp = test::TestRequest::post().uri(&format!("/users")).append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).set_json(&request_body).send_request(&app).await;
        assert!(resp.status().is_success());
        let user: UserDTO = test::read_body_json(resp).await;
        assert_eq!(user.username, "username");

        // Get test
        let resp = test::TestRequest::get().uri(&format!("/users/{}", user.id)).append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).send_request(&app).await;
        assert!(resp.status().is_success());
        let retrieved_user: UserDTO = test::read_body_json(resp).await;
        assert_eq!(retrieved_user.username, "username");

        // Get all test
        let req = test::TestRequest::get().uri("/users").append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let users: ResultPaging<UserDTO> = test::read_body_json(resp).await;
        assert_eq!(users.items.len(), 2);

        // Delete test
        let resp = test::TestRequest::delete().uri(&format!("/users/{}", user.id)).append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).send_request(&app).await;
        assert!(resp.status().is_success());

        // Get all test
        let req = test::TestRequest::get().uri("/users").append_header((
            actix_web::http::header::AUTHORIZATION,
            HeaderValue::from_str(&token).unwrap(),
        )).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let users: ResultPaging<UserDTO> = test::read_body_json(resp).await;
        assert_eq!(users.items.len(), 1);
    }
}
