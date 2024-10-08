use super::jwt::Claims;
use std::sync::Arc;
use diesel::{insert_into, update};
use diesel::prelude::*;
use diesel::result::Error;
use jsonwebtoken::TokenData;
use log::info;
use crate::domain::error::CommonError;
use crate::domain::models::service_context::ServiceContext;
use crate::domain::services::jwt::JwtService;
use crate::domain::services::service_context::ServiceContextService;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::service_context::ServiceContextDiesel;
use actix_web::{HttpMessage, HttpRequest};


#[derive(Clone)]
pub struct ServiceContextServiceImpl {
    pub pool: Arc<DBConn>,
    pub jwt_service: Arc<dyn JwtService>
}

impl ServiceContextServiceImpl {
    pub fn new(db: Arc<DBConn>, jwt_service: Arc<dyn JwtService>) -> Self {
        ServiceContextServiceImpl {
            pool: db,
            jwt_service: jwt_service,
        }
    }

    fn get_service_context(&self) -> ServiceContext {
        use crate::infrastructure::schema::service_contexts::dsl::{id, service_contexts};
        let mut conn = self.pool.get().unwrap();
        let result: Result<ServiceContextDiesel, Error> = service_contexts.filter(id.eq(1)).first::<ServiceContextDiesel>(&mut conn);

        if result.is_err() {
            info!("Service context does not exist, creating a service context...");
            return self.create_service_context();
        }

        result.unwrap().into()
    }

    fn create_service_context(&self) -> ServiceContext {
        use crate::infrastructure::schema::service_contexts::dsl::service_contexts;
        let mut conn = self.pool.get().unwrap();
        let result: Result<ServiceContextDiesel, Error> = insert_into(service_contexts).values(ServiceContextDiesel { id: 1, maintenance: false }).get_result(&mut conn);

        if result.is_err() {
            panic!("Could not create service context");
        }
        result.unwrap().into()
    }
}

impl ServiceContextService for ServiceContextServiceImpl {

    fn get_claims(&self, request: HttpRequest) -> Option<Claims> {
        return request.extensions_mut().get::<Claims>().cloned()
    }

    fn get_service_context(&self) -> ServiceContext {
        self.get_service_context()
    }

    fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, CommonError> {
        return self.jwt_service.validate_token(token);
    }

    fn update(&self, service_context: ServiceContext) -> ServiceContext {
        let service_context_diesel: ServiceContextDiesel = ServiceContextDiesel::from(service_context);
        let mut conn = self.pool.get().unwrap();
        use crate::infrastructure::schema::service_contexts::dsl::{service_contexts, id};
        let result: Result<ServiceContextDiesel, Error> = update(service_contexts)
            .filter(id.eq(1)).set(service_context_diesel).get_result(&mut conn);

        if result.is_err() {
            panic!("Could not update service context");
        }
        result.unwrap().into()
    }

    fn is_maintenance_active(&self) -> bool {
        self.get_service_context().maintenance
    }
}
