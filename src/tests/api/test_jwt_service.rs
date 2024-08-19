

#[cfg(test)]
mod test_todo_controllers{
    use std::env;
    use std::sync::Arc;
    use actix_clean_architecture::{api::dto::user::{LoggedUserDTO, LoginUserDTO}, domain::{constants::SECRET_JWT, services::jwt::JwtService}, infrastructure::{models::user::UserRoleFormat, services::jwt::JwtServiceImpl}};
    use actix_web::{http::header::HeaderValue, test};
    //use testcontainers::clients;
    //use testcontainers::images::postgres;
    use actix_clean_architecture::infrastructure::databases::postgresql::db_pool;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use actix_clean_architecture::{container::Container, create_app::create_app};
    use chrono::Duration;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    #[actix_web::test]
    async fn test() {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();

        //let docker = clients::Cli::default();
        //let postgres_node = docker.run(postgres::Postgres::default());

        /*
        
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres", postgres_node.get_host_port_ipv4(5432)
        );

        env::set_var(POSTGRESQL_DB_URI, connection_string);
         */
        
        {
          let pool = Arc::new(db_pool());
          pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();
        }

        let container = Arc::new(Container::new());

        let app = test::init_service(create_app(container)).await;

        // check if authorization is required
        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), false);

        // Check if empty table
        let req = test::TestRequest::post().uri("/login").set_json(LoginUserDTO{
          email: String::from("admin@admin.com"),
          password: String::from("Azerty95"),
          refresh_token: String::new(),
        }).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let logged_user: LoggedUserDTO = test::read_body_json(resp).await;
        
        assert_eq!(logged_user.token.trim().is_empty(), false);

        // create token Player
        let secret = env::var(SECRET_JWT)
            .expect(&*format!("{value} must be set", value = SECRET_JWT));
        let jwt_service = JwtServiceImpl::new(secret);

        let jwt = jwt_service.create_token(1, UserRoleFormat::Player.into(), Duration::hours(1), false).unwrap();
        let token = format!("Bearer {}", jwt);

        // check if authorization work after login
        let req = test::TestRequest::get().uri("/users").append_header((
          actix_web::http::header::AUTHORIZATION,
          HeaderValue::from_str(&token).unwrap(),
      )).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);


    }
}
