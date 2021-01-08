#[allow(non_snake_case)]
pub mod RecurringService {
  use crate::common::errors::ApiError;
  use crate::controllers::recurring_controller::RecurringNewPayload;
  use crate::models::{recurring_model::*, user_model::User};
  use crate::services::users::UserService;
  use actix_web::web::Data;
  use wither::{mongodb::bson::oid::ObjectId, Model};

  pub async fn get_recurring(recurring_id: String, user: User) -> Result<Recurring, ApiError> {
    let recurring_id_opt = Some(
      ObjectId::with_string(recurring_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

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

  // not necessary
  // pub async fn get_recurrings() -> Result<Vec<Recurring>, ApiError> {}

  pub async fn new_recurring(
    payload: RecurringNewPayload,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<Recurring, ApiError> {
    let mut recurring: Recurring = payload.into();
    recurring.set_id(ObjectId::new());

    user.recurrings.push(recurring.clone());

    user_service.save(user).await?;

    Ok(recurring)
  }

  pub async fn update_recurring(
    recurring_id: String,
    payload: RecurringNewPayload,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<Recurring, ApiError> {
    let recurring_id_opt = Some(
      ObjectId::with_string(recurring_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

    let mut recurring: Recurring = payload.into();
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
      })?;

    user_service.save(user).await?;

    Ok(updated)
  }
  pub async fn delete_recurring(
    recurring_id: String,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<Recurring, ApiError> {
    let recurring_id_opt = Some(
      ObjectId::with_string(recurring_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

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
      .and_then(|pos| Ok(user.recurrings.swap_remove(pos)))?;

    user_service.save(user).await?;

    Ok(removed)
  }
}
