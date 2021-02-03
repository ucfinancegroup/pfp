use crate::models::user_model::User;
use crate::services::users::UserService;
use actix_web::{
  get, put,
  web::{Data, Path},
  HttpResponse,
};

#[get("/insights")]
pub async fn get_insight(user: User) -> HttpResponse {
  crate::common::into_response(user.get_non_dismissed_insights())
}

#[put("/insight/{id}/dismiss")]
pub async fn dismiss_insight(
  mut user: User,
  user_service: Data<UserService>,
  id: Path<String>,
) -> HttpResponse {
  // cant dismiss an incomplete insight
  crate::common::into_response_res(
    user_service
      .dismiss_insight(&mut user, id.into_inner())
      .await,
  )
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_insight);
  config.service(dismiss_insight);
}
