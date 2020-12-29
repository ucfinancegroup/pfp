use crate::common::errors::ApiError;
use crate::common::Validation;
use crate::controllers::user_controller::{LoginPayload, SignupPayload};
use crate::models::{
  session_model,
  user_model::{PlaidItem, User},
};
use crate::services::db;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::sync::Collection;

#[derive(Clone)]
pub struct UserService {
  col: Collection,
}

impl UserService {
  pub fn new(db: &db::DatabaseService) -> UserService {
    UserService {
      col: db.collection(db::Collections::Users),
    }
  }

  pub fn signup(&self, data: SignupPayload) -> Result<User, ApiError> {
    if let Err(e) = data.validate() {
      return Err(ApiError::new(400, e));
    }

    // check for unused email
    if let Ok(Some(_)) = self
      .col
      .find_one(Some(doc! {"email": data.email.clone()}), None)
    {
      return Err(ApiError::new(400, "Email is in use".to_string()));
    }

    User::hash_password(data.password.clone())
      .map_err(|_| ApiError::new(400, "Password hashing failed".to_string()))
      .and_then(|password_hash| {
        let user = User {
          _id: ObjectId::new(),
          email: data.email,
          password: password_hash,
          first_name: data.first_name,
          last_name: data.last_name,
          income: data.income,
          accounts: None,
        };

        self
          .col
          .insert_one(crate::common::into_bson_document(&user), None)
          .and_then(|_| Ok(user))
          .map_err(|_| ApiError::new(500, "Database Error".to_string()))
      })
  }

  pub fn login(&self, data: LoginPayload) -> Result<User, ApiError> {
    if let Err(e) = data.validate() {
      return Err(ApiError::new(400, e));
    }

    // search db for user
    let search_db_res = self
      .col
      .find_one(Some(doc! {"email": data.email.clone()}), None)
      .map_err(|_| ApiError::new(500, "DB Error".to_string()));

    // check if user found and parse to User
    let got_user_res: Result<User, ApiError> = search_db_res.and_then(|user_opt| {
      user_opt
        .ok_or(ApiError::new(500, "User not found".to_string()))
        .and_then(|user| {
          bson::from_bson(user.into())
            .map_err(|_| ApiError::new(500, "user format error".to_string()))
        })
    });

    // verify password, return user if good
    got_user_res.and_then(|user| {
      user
        .compare_password(data.password)
        .and_then(|is_correct_password| {
          if is_correct_password {
            Ok(user)
          } else {
            Err(ApiError::new(401, "Incorrect user or password".to_string()))
          }
        })
    })
  }

  pub fn new_from_session(&self, session: session_model::Session) -> Result<User, ApiError> {
    self
      .col
      .find_one(Some(doc! {"_id": session.user_id.clone()}), None)
      .map_err(|_| ApiError::new(500, "DB Error".to_string()))
      .and_then(|user_opt| {
        user_opt
          .ok_or(ApiError::new(500, "User not found".to_string()))
          .and_then(|user| {
            bson::from_bson(user.into())
              .map_err(|_| ApiError::new(500, "user format error".to_string()))
          })
      })
  }

  pub fn add_new_account(
    &self,
    user: &User,
    access_token: String,
    item_id: String,
  ) -> Result<(), ApiError> {
    self.col
      .update_one(
        doc! {"_id": user._id.clone()},
        doc! {"$push": doc!{"accounts" : bson::to_bson(&PlaidItem::new(item_id, access_token)).unwrap()}},
        None,
      )
      .map_err(|_| ApiError::new(500, "Database Error".to_string()))
      .and_then(|_| Ok(()))
  }
}
