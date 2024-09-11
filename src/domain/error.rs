use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl CommonError {
    pub fn entity_already_exist() -> Self {
        CommonError {
            code: 2,
            message: String::from("entity_already_exist"),
        }
    }

    pub fn session_error() -> Self {
        CommonError {
            code: 4,
            message: String::from("session_error"),
        }
    }

    pub fn bad_connection() -> Self {
        CommonError {
            code: 3,
            message: String::from("bad_connection"),
        }
    }

}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl Into<CommonError> for jsonwebtoken::errors::Error {
    fn into(self) -> CommonError {
        CommonError {
            message: self.to_string(),
            code: 2,
        }
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}
