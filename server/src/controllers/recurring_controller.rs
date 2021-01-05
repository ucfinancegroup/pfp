use crate::common::errors::ApiError;
use crate::models::{recurring_model::*, user_model::User};
use crate::services::{sessions::SessionService, users::UserService};
use actix_session::Session;
use actix_web::{
  delete, get, post, put,
  web::{Data, Path},
  HttpResponse,
};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use wither::{mongodb::bson::oid::ObjectId, Model};

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
  pub principal: i64,
  #[serde(rename = "amount")]
  pub amount: i64,
  #[serde(rename = "interest")]
  #[validate(range(min = 0))]
  pub interest: f32,
  #[serde(rename = "frequency")]
  pub frequency: TimeInterval,
}

fn validate_recurring_new_payload(data: &RecurringNewPayload) -> Result<(), ValidationError> {
  if data.principal == 0 && data.interest == 0.0 && data.amount != 0 {
    Ok(())
  } else if data.amount == 0 && data.principal != 0 {
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
pub async fn get_recurring(
  session: Session,
  Path(recurring_id): Path<String>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let user: User = user_service.new_from_session(finch_session).await.unwrap();

      let recurring_id_opt = ObjectId::with_string(recurring_id.as_str()).ok();

      let found = user
        .recurrings
        .into_iter()
        .find(|rec| rec.id == recurring_id_opt)
        .clone();

      found.ok_or(ApiError::new(
        400,
        format!(
          "No recurring with id {} found in current user",
          recurring_id
        ),
      ))
    }
  })
}

#[get("/recurrings")]
pub async fn get_recurrings(
  session: Session,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let user: User = user_service.new_from_session(finch_session).await.unwrap();
      Ok(user.recurrings)
    }
  })
}

#[post("/recurring/new")]
pub async fn new_recurring(
  session: Session,
  payload: Json<RecurringNewPayload>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let mut user: User = user_service.new_from_session(finch_session).await.unwrap();

      let mut recurring: Recurring = payload.into_inner().into();
      recurring.set_id(ObjectId::new());

      user.recurrings.push(recurring.clone());

      user_service.save(user).await.and_then(|_| Ok(recurring))
    }
  })
}

#[put("/recurring/{id}")]
pub async fn update_recurring(
  session: Session,
  payload: Json<RecurringNewPayload>,
  Path(recurring_id): Path<String>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let mut user: User = user_service.new_from_session(finch_session).await.unwrap();

      let recurring_id_opt = ObjectId::with_string(recurring_id.as_str()).ok();

      match recurring_id_opt {
        Some(_) => (),
        None => return ApiError::new(400, "Malformed object id".to_string()).into(),
      };

      let mut recurring: Recurring = payload.into_inner().into();
      recurring.set_id(recurring_id_opt.unwrap().clone());

      let updated = user
        .recurrings
        .iter_mut()
        .find(|rec| rec.id == recurring.id)
        .ok_or(ApiError::new(
          400,
          format!(
            "No recurring with id {} found in current user",
            recurring_id
          ),
        ))
        .and_then(|rec| {
          *rec = recurring.clone();
          Ok(recurring)
        });

      user_service.save(user).await.and_then(move |_| updated)
    }
  })
}

#[delete("/recurring/{id}")]
pub async fn delete_recurring(
  session: Session,
  Path(recurring_id): Path<String>,
  user_service: Data<UserService>,
  session_service: Data<SessionService>,
) -> HttpResponse {
  crate::common::into_response_res(match session_service.get_valid_session(&session).await {
    Err(e) => Err(e),
    Ok(finch_session) => {
      let mut user: User = user_service.new_from_session(finch_session).await.unwrap();

      let recurring_id_opt = ObjectId::with_string(recurring_id.as_str()).ok();

      let removed = user
        .recurrings
        .iter()
        .position(|rec| rec.id == recurring_id_opt)
        .ok_or(ApiError::new(
          400,
          format!(
            "No recurring with id {} found in current user",
            recurring_id
          ),
        ))
        .and_then(|pos| Ok(user.recurrings.swap_remove(pos)));

      user_service.save(user).await.and_then(move |_| removed)
    }
  })
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
  config.service(new_recurring);
  config.service(delete_recurring);
  config.service(update_recurring);
  config.service(get_recurring);
  config.service(get_recurrings);
}
