use crate::models::{recurring_model::*, user_model::User};
use crate::services::{recurrings::RecurringService, users::UserService};
use actix_web::{
  delete, get, post, put,
  web::{Data, Path},
  HttpResponse,
};
use actix_web_validator::{Json, Validate};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Validate, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[validate(schema(
  function = "validate_recurring_new_payload",
  skip_on_field_errors = false
))]
pub struct RecurringNewPayload {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "start")]
  #[validate(range(min = 0))]
  pub start: i64,
  #[serde(rename = "end")]
  #[validate(range(min = 0))]
  pub end: i64,
  #[serde(rename = "principal")]
  pub principal: Decimal,
  #[serde(rename = "amount")]
  pub amount: Decimal,
  #[serde(rename = "interest")]
  #[validate(custom = "crate::common::decimal_at_least_zero")]
  pub interest: Decimal,
  #[serde(rename = "frequency")]
  pub frequency: TimeInterval,
}

fn validate_recurring_new_payload(data: &RecurringNewPayload) -> Result<(), ValidationError> {
  if data.principal == dec!(0) && data.interest == dec!(0) && data.amount != dec!(0) {
    Ok(())
  } else if data.amount == dec!(0) && data.principal != dec!(0) {
    Ok(())
  } else {
    Err(ValidationError::new(
      "Only one of Principal and Amount can be non-zero\
     and Interest must be zero if Amount is non-zero.",
    ))
  }
}

impl Into<Recurring> for RecurringNewPayload {
  fn into(self) -> Recurring {
    Recurring {
      id: None,
      name: self.name,
      start: self.start,
      end: self.end,
      principal: self.principal,
      amount: self.amount,
      interest: self.interest,
      frequency: self.frequency,
    }
  }
}

#[get("/recurring/{id}")]
pub async fn get_recurring(Path(recurring_id): Path<String>, user: User) -> HttpResponse {
  crate::common::into_response_res(RecurringService::get_recurring(recurring_id, user).await)
}

#[get("/recurrings")]
pub async fn get_recurrings(user: User) -> HttpResponse {
  crate::common::into_response(user.recurrings)
}

#[post("/recurring/new")]
pub async fn new_recurring(
  payload: Json<RecurringNewPayload>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  crate::common::into_response_res(
    RecurringService::new_recurring(payload.into_inner(), user, user_service).await,
  )
}

#[put("/recurring/{id}")]
pub async fn update_recurring(
  payload: Json<RecurringNewPayload>,
  Path(recurring_id): Path<String>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  crate::common::into_response_res(
    RecurringService::update_recurring(recurring_id, payload.into_inner(), user, user_service)
      .await,
  )
}

#[delete("/recurring/{id}")]
pub async fn delete_recurring(
  Path(recurring_id): Path<String>,
  user: User,
  user_service: Data<UserService>,
) -> HttpResponse {
  crate::common::into_response_res(
    RecurringService::delete_recurring(recurring_id, user, user_service).await,
  )
}

// "examples" can never be an ID, and we will put the service ahead of the others
// currently, we take a User to make this an authorised route... Should we?
#[get("/recurring/examples")]
pub async fn get_recurring_examples(_: User) -> HttpResponse {
  crate::common::into_response(vec![
    RecurringNewPayload {
      name: "Unemployment Benefits".to_string(),
      start: 1609977600,
      end: 1617753600,
      principal: dec!(0),
      interest: dec!(0),
      amount: dec!(300),
      frequency: TimeInterval {
        typ: Typ::Monthly,
        content: 1,
      },
    },
    RecurringNewPayload {
      name: "Pay Babysitter".to_string(),
      start: 1609977600,
      end: 1617753600,
      principal: dec!(0),
      interest: dec!(0),
      amount: dec!(-60),
      frequency: TimeInterval {
        typ: Typ::Monthly,
        content: 1,
      },
    },
  ])
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(get_recurring_examples);
  config.service(new_recurring);
  config.service(delete_recurring);
  config.service(update_recurring);
  config.service(get_recurring);
  config.service(get_recurrings);
}
