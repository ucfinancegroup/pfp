use crate::common::errors::ApiError;
use crate::models::financial_product_model::*;
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

  pub async fn resolve_account(&self, account: &Account) -> Result<FinancialProduct, ApiError> {
    let name = account
      .official_name
      .clone()
      .or(Some(account.name.clone()))
      .ok_or(ApiError::new(
        500,
        "Could not get name for Account".to_string(),
      ))?;

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
