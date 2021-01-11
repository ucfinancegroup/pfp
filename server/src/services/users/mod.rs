use crate::common::errors::ApiError;
use crate::controllers::user_controller::{LoginPayload, SignupPayload};
use crate::models::{
  session_model,
  user_model::{PlaidItem, Snapshot, User},
};
use crate::services::{db, finchplaid::ApiClient, snapshots::SnapshotService};
use actix_web::web::Data;
use std::sync::{Arc, Mutex};
use wither::{
  mongodb::{bson::doc, Database},
  prelude::Migrating,
  Model,
};

#[derive(Clone)]
pub struct UserService {
  db: Database,
}

impl UserService {
  pub async fn new(db: &db::DatabaseService) -> UserService {
    let _ = User::migrate(&db.db).await.unwrap();
    UserService { db: db.db.clone() }
  }

  pub async fn signup(&self, data: SignupPayload) -> Result<User, ApiError> {
    // check for unused email
    if let Ok(Some(_)) =
      User::find_one(&self.db, Some(doc! {"email": data.email.clone()}), None).await
    {
      return Err(ApiError::new(400, "Email is in use".to_string()));
    }

    let password_hash = User::hash_password(data.password.clone())
      .map_err(|_| ApiError::new(400, "Password hashing failed".to_string()))?;

    let mut user = User {
      id: None,
      email: data.email,
      password: password_hash,
      first_name: data.first_name,
      last_name: data.last_name,
      income: data.income,
      accounts: vec![],
      snapshots: vec![],
      recurrings: vec![],
      goals: vec![],
    };

    user.save(&self.db, None).await.map_or_else(
      |_| Err(ApiError::new(500, "Database Error".to_string())),
      |_| Ok(user),
    )
  }

  pub async fn login(&self, data: LoginPayload) -> Result<User, ApiError> {
    // search db for user
    let search_db_res = User::find_one(&self.db, Some(doc! {"email": data.email.clone()}), None)
      .await
      .map_err(|_| ApiError::new(500, "DB Error".to_string()));

    // check if user found and parse to User
    let got_user_res: Result<User, ApiError> = search_db_res
      .and_then(|user_opt| user_opt.ok_or(ApiError::new(500, "User not found".to_string())));

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

  pub async fn new_from_session(&self, session: session_model::Session) -> Result<User, ApiError> {
    User::find_one(&self.db, Some(doc! {"_id": session.user_id.clone()}), None)
      .await
      .map_err(|_| ApiError::new(500, "DB Error".to_string()))
      .and_then(|user_opt| user_opt.ok_or(ApiError::new(500, "User not found".to_string())))
  }

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

  pub async fn get_snapshots(
    &self,
    user: &mut User,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<Vec<Snapshot>, ApiError> {
    if SnapshotService::need_new_snapshot(&user.snapshots) {
      SnapshotService::add_new_snapshot(user, plaid_client).await?;
      user.save(&self.db, None).await.map_err(|_| {
        ApiError::new(
          500,
          "Could not save user to database after Snapshot".to_string(),
        )
      })?;
    }
    Ok(user.snapshots.clone())
  }

  pub async fn save(&self, mut u: User) -> Result<(), ApiError> {
    u.save(&self.db, None)
      .await
      .map_err(|_| ApiError::new(500, "Database Error".to_string()))
  }
}
