pub mod AccountService {
  use crate::common::errors::ApiError;
  use crate::models::user_model::{PlaidItem, User};
  use crate::services::finchplaid::ApiClient;
  use actix_web::web::Data;

  pub async fn add_new_account(
    &self,
    user: User,
    access_token: String,
    item_id: String,
  ) -> Result<(), ApiError> {
    user.update(&self.db, None, doc! {"$push": doc!{"accounts" : crate::common::into_bson_document(&PlaidItem{item_id, access_token})}}, None).await
    .map_err(|_| ApiError::new(500, "Database Error".to_string()))
    .and_then(|_| Ok(()))
  }

  
}