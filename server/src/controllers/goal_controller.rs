use crate::models::{goal_model::*, user_model::User};
use crate::services::{goals::GoalService, users::UserService};
use actix_web::{
  delete, get, post, put,
  web::{Data, Path},
  HttpResponse,
};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[validate(schema(function = "validate_goal_new_payload", skip_on_field_errors = false))]
pub struct GoalNewPayload {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "start")]
  pub start: i64,
  #[serde(rename = "end")]
  pub end: i64,
  #[serde(rename = "threshold")]
  pub threshold: i64,
  #[serde(rename = "metric")]
  pub metric: GoalMetrics,
}

fn validate_goal_new_payload(data: &GoalNewPayload) -> Result<(), ValidationError> {
  if data.start >= data.end {
    return Err(ValidationError::new(
      "Start time of Goal should be strictly before End time.",
    ));
  }

  Ok(())
}

impl Into<Goal> for GoalNewPayload {
  fn into(self) -> Goal {
    Goal {
      id: None,
      name: self.name,
      start: self.start,
      end: self.end,
      threshold: self.threshold,
      metric: self.metric,
    }
  }
}

#[post("/goal/new")]
pub async fn new_goal(
  user: User,
  user_service: Data<UserService>,
  payload: Json<GoalNewPayload>,
) -> HttpResponse {
  crate::common::into_response_res(
    GoalService::new_goal(payload.into_inner(), user, user_service).await,
  )
}

#[get("/goals")]
pub async fn get_goals(user: User) -> HttpResponse {
  crate::common::into_response(user.goals)
}

#[get("/goal/{id}")]
pub async fn get_goal(user: User, id: Path<String>) -> HttpResponse {
  crate::common::into_response_res(GoalService::get_goal(id.into_inner(), user).await)
}

#[put("/goal/{id}")]
pub async fn update_goal(
  user: User,
  user_service: Data<UserService>,
  id: Path<String>,
  payload: Json<GoalNewPayload>,
) -> HttpResponse {
  crate::common::into_response_res(
    GoalService::update_goal(id.into_inner(), payload.into_inner(), user, user_service).await,
  )
}

#[delete("/goal/{id}")]
pub async fn delete_goal(
  user: User,
  user_service: Data<UserService>,
  id: Path<String>,
) -> HttpResponse {
  crate::common::into_response_res(
    GoalService::delete_goal(id.into_inner(), user, user_service).await,
  )
}

// "examples" can never be an ID, and we will put the service ahead of the others
// currently, we take a User to make this an authorised route... Should we?
#[get("/goal/examples")]
pub async fn get_goal_examples(_: User) -> HttpResponse {
  crate::common::into_response(vec![
    GoalNewPayload {
      name: "Save 100 Dollars".to_string(),
      start: 1609977600,
      end: 1617753600,
      threshold: 10000, // dollars times 100
      metric: GoalMetrics::Savings,
    },
    GoalNewPayload {
      name: "Spend under 100 Dollars".to_string(),
      start: 1609977600,
      end: 1617753600,
      threshold: -10000, // dollars times 100
      metric: GoalMetrics::Spending,
    },
  ])
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_goal_examples);
  config.service(get_goal);
  config.service(get_goals);
  config.service(new_goal);
  config.service(delete_goal);
  config.service(update_goal);
}
