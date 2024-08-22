use actix_web::{web, Result};
use crate::api::dto::user::{LoggedUserDTO, LoginUserDTO};
use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;

pub async  fn login_user_handler(
  user_service: web::Data<dyn UserService>, post_data: web::Json<LoginUserDTO>
) -> Result<web::Json<LoggedUserDTO>, ApiError> {
  let logged_in_user = user_service.login(post_data.into_inner().into()).await?;

  Ok(web::Json(logged_in_user.into()))
}