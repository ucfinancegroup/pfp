use crate::common::errors::ApiError;
use crate::controllers::plaid_controller::AccountResponse;
use crate::controllers::user_controller::{LoginPayload, SignupPayload, UpdatePayload};
use crate::models::{
  session_model,
  user_model::{PlaidItem, Snapshot, User},
};
use crate::services::{db, finchplaid::ApiClient, snapshots::SnapshotService};
use actix_web::web::Data;
use serde_json::{json, Map, Value};
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
      location: data.location,
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

  pub async fn update(&self, mut user: User, data: UpdatePayload) -> Result<User, ApiError> {
    if let Some(email) = data.email {
      user.email = email;
    }
    if let Some(password) = data.password {
      let password_hash = User::hash_password(password.clone())
        .map_err(|_| ApiError::new(400, "Password hashing failed".to_string()))?;

      user.password = password_hash;
    }
    if let Some(first_name) = data.first_name {
      user.first_name = first_name;
    }
    if let Some(last_name) = data.last_name {
      user.last_name = last_name;
    }
    if let Some(income) = data.income {
      user.income = income;
    }
    if let Some(location) = data.location {
      user.location = location;
    }

    user.save(&self.db, None).await.map_or_else(
      |_| Err(ApiError::new(500, "Database Error".to_string())),
      |_| Ok(user),
    )
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

  pub async fn get_accounts(
    &self,
    user: User,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<Map<String, Value>, ApiError> {
    let mut res = Map::new();

    for item in user.accounts.iter() {
      match crate::services::finchplaid::get_net_worth(item, plaid_client.clone()).await {
        Ok(num) => res.insert(
          item.item_id.clone(),
          json!(AccountResponse { balance: num }),
        ),
        Err(e) => res.insert(item.item_id.clone(), json!(e)),
      };
    }
    Ok(res)
  }

  pub async fn delete_account(&self, account_id: String, user: User) -> Result<(), ApiError> {
    match UserService::delete(account_id, user) {
      Ok(mut res) => res
        .save(&self.db, None)
        .await
        .map_err(|_| ApiError::new(500, "Database Error".to_string()))
        .and_then(|_| Ok(())),
      Err(e) => Err(e),
    }
  }

  pub fn delete(account_id: String, mut user: User) -> Result<User, ApiError> {
    user
      .accounts
      .iter()
      .position(|rec| rec.item_id == account_id)
      .ok_or(ApiError::new(
        400,
        format!("No account with id {} found in current user", account_id),
      ))
      .and_then(|pos| Ok(user.accounts.swap_remove(pos)))?;

    return Ok(user);
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

  pub async fn save(&self, u: &mut User) -> Result<(), ApiError> {
    u.save(&self.db, None)
      .await
      .map_err(|_| ApiError::new(500, "Database Error".to_string()))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use std::error::Error;
  use std::fs::File;
  use std::io::BufReader;

  use plaid::models::RetrieveTransactionsResponse;

  fn load_test_data() -> Result<RetrieveTransactionsResponse, Box<dyn Error>> {
    let file = File::open("./tests/test_snapshots.json")?;
    let reader = BufReader::new(file);
    let transactions = serde_json::from_reader(reader)?;
    Ok(transactions)
  }

  #[test]
  fn test_delete_account() {
    let accounts = load_test_data().unwrap();

    let to_delete = PlaidItem {
      item_id: accounts.accounts[0].account_id.clone(),
      access_token: String::from("12345"),
    };
    let mut accounts_array: Vec<PlaidItem> = Vec::new();
    accounts_array.push(to_delete);

    let user = User {
      id: None,
      email: String::from("test@test.com"),
      password: String::from("test@test.com"),
      first_name: String::from("fn"),
      last_name: String::from("ln"),
      income: 0.0,
      accounts: accounts_array,
      snapshots: Vec::new(),
      recurrings: Vec::new(),
      goals: Vec::new(),
    };

    let mut found = false;

    let obj = match UserService::delete(accounts.accounts[0].account_id.clone(), user) {
      Ok(new_user) => new_user,

      Err(_) => return assert_eq!(false, true),
    };

    for account in obj.accounts.iter() {
      if account.item_id.eq(&accounts.accounts[0].account_id) {
        found = true;
        break;
      }
    }

    assert_eq!(false, found);
  }
}
