use crate::common::errors::ApiError;
use crate::models::{financial_product_model::*, user_model::AccountRecord};
use crate::services::db::DatabaseService;
use plaid::models::Account;
use wither::{
  mongodb::{bson::doc, Database},
  prelude::Migrating,
  Model,
};

#[derive(Clone)]
pub struct FinProductService {
  db: Database,
}

impl FinProductService {
  pub async fn new(db: &DatabaseService) -> FinProductService {
    let _ = FinancialProduct::migrate(&db.db).await.unwrap();
    FinProductService { db: db.db.clone() }
  }

  pub fn make_account_record(
    item_id: String,
    account: &Account,
    product: &Option<FinancialProduct>,
  ) -> Result<AccountRecord, ApiError> {
    Ok(AccountRecord {
      item_id,
      account_id: account.account_id.clone(),
      account_name: Self::get_account_name(account)?,
      hidden: false,
      known_account_id: product.as_ref().map(|p| p.id()).flatten(),
    })
  }

  fn get_account_name(account: &Account) -> Result<String, ApiError> {
    account
      .official_name
      .clone()
      .or(Some(account.name.clone()))
      .ok_or(ApiError::new(
        500,
        "Could not get name for Account".to_string(),
      ))
  }

  pub async fn resolve_account(&self, account: &Account) -> Result<FinancialProduct, ApiError> {
    let name = Self::get_account_name(account)?;

    self.resolve_with_name(&name).await
  }

  pub async fn resolve_with_name(&self, name: &str) -> Result<FinancialProduct, ApiError> {
    FinancialProduct::find_one(&self.db, doc! {"name": name.clone()}, None)
      .await
      .map_err(|_| ApiError::new(500, "Database Error".to_string()))?
      .ok_or(ApiError::new(
        500,
        format!(
          "Could not find account with name {} in Financial Products collection",
          name
        ),
      ))
  }
}
