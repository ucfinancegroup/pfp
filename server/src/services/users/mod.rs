use crate::common::errors::ApiError;
use crate::controllers::plaid_controller::{AccountError, AccountResponse};
use crate::controllers::user_controller::{LoginPayload, SignupPayload, UpdatePayload};
use crate::models::{
  insight_model::Insight,
  session_model,
  user_model::{PlaidItem, Snapshot, User},
};
use crate::services::{
  db, financial_products::FinProductService, finchplaid::ApiClient, snapshots::SnapshotService,
};
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

  pub async fn email_in_use(&self, email: &String) -> Result<bool, ApiError> {
    User::find_one(&self.db, Some(doc! {"email": email}), None)
      .await
      .map_or_else(
        |_| Err(ApiError::new(500, "Db Error".to_string())),
        |good| Ok(good.is_some()),
      )
  }

  pub async fn signup(&self, data: SignupPayload) -> Result<User, ApiError> {
    if self.email_in_use(&data.email).await? {
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
      birthday: data.birthday,
      accounts: vec![],
      account_records: vec![],
      snapshots: vec![],
      recurrings: vec![],
      goals: vec![],
      insights: vec![],
      plans: vec![],
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
    if let Some(birthday) = data.birthday {
      user.birthday = birthday;
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
    mut user: User,
    access_token: String,
    item_id: String,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
    fin_product_service: Data<FinProductService>,
  ) -> Result<(), ApiError> {
    user.accounts.push(PlaidItem {
      item_id: item_id.clone(),
      access_token,
    });
    self.save(&mut user).await?;

    let accounts_info =
      crate::services::finchplaid::get_item_accounts(&user.accounts.last().unwrap(), plaid_client)
        .await;
    let accounts = accounts_info?.accounts;

    let mut state = Ok(());
    for account in accounts.iter() {
      let found_product = fin_product_service.resolve_account(&account).await;

      let res =
        FinProductService::make_account_record(item_id.clone(), account, &found_product.ok())
          .and_then(|record| {
            user.account_records.push(record);
            Ok(())
          });

      state = state.and(res);
    }

    self.save(&mut user).await?;

    state
  }

  pub async fn get_accounts(
    &self,
    user: &User,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<AccountResponse, ApiError> {
    let mut account_successes = Vec::new();
    let mut account_errors = Vec::new();

    for item in user.accounts.iter() {
      match crate::services::finchplaid::get_account_data(item, plaid_client.clone()).await {
        Ok(mut res) => account_successes.append(&mut res),
        Err(_) => account_errors.push(AccountError {
          item_id: item.item_id.clone(),
          code: 500,
          message: "Failed to obtain account information with given id".to_string(),
        }),
      };
    }

    Ok(AccountResponse {
      accounts: account_successes,
      account_errors: account_errors,
    })
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

  pub async fn dismiss_insight(
    &self,
    user: &mut User,
    insight_id: String,
  ) -> Result<Insight, ApiError> {
    let insight_id = wither::mongodb::bson::oid::ObjectId::with_string(insight_id.as_str())
      .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?;
    let insight_id_opt = Some(insight_id.clone());

    let updated = user
      .insights
      .iter_mut()
      .find(|rec| rec.id == insight_id_opt)
      .ok_or(ApiError::new(
        400,
        format!("No insight with id {} found in current user", insight_id),
      ))
      .and_then(|rec: &mut Insight| {
        rec.dismissed = true;
        Ok(rec)
      })?
      .clone();

    self.save(user).await.and_then(|_| Ok(updated))
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

  use crate::models::user_model::Location;
  use plaid::models::RetrieveTransactionsResponse;
  use rust_decimal_macros::dec;
  use std::error::Error;
  use std::fs::File;
  use std::io::BufReader;

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
      income: dec!(0.0),
      location: Location {
        ..Default::default()
      },
      birthday: "1970-01-01".to_string(),
      accounts: accounts_array,
      account_records: vec![],
      snapshots: Vec::new(),
      recurrings: Vec::new(),
      goals: Vec::new(),
      insights: Vec::new(),
      plans: Vec::new(),
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
