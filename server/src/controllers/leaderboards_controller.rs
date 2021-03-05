use crate::common::Examples;
use crate::models::{insight_model::Insight, user_model::User};
use crate::services::users::UserService;
use actix_web::{
  get, put,
  web::{Data, Path},
  HttpResponse,
};

#[get("/leaderboard/{board}")]
pub async fn get_leaderboard(user: User, board: Path<String>) -> HttpResponse {
  println!("{}", board.into_inner());
  let res: Result<String, ()> = Ok("test".to_string());
  crate::common::into_response(res)
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_leaderboard);
}
