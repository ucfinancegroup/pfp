use crate::common::errors::AppError;
use crate::models::{
  insight_model::{Insight, InsightTypes},
  user_model::User,
};
use crate::services::{db::DatabaseService, insights::common::match_income_range};
use wither::{
  mongodb::{
    bson::{self, doc},
    Collection,
  },
  Model,
};

pub fn project_account_records() -> bson::Document {
  doc! {
    "$project": {
      "account_records": 1,
    }
  }
}

pub async fn generate_similar_user_insight(
  user: &User,
  db_service: &DatabaseService,
) -> Result<Insight, AppError> {
  let agg = User::collection(&db_service.db)
    .aggregate(
      vec![match_income_range(&user), project_account_records()],
      None,
    )
    .await
    .map_err(|_| AppError::new("Error during aggregation"))?;

  Err(AppError::new(""))

  // Ok(Insight::new(
  //   "Savings Insight".to_string(),
  //   format!(
  //     "Your savings over the last {} days puts you above {}% of similar users!",
  //     lookback.num_days(),
  //     100 * metrics.savings_less / metrics.total_similar_users
  //   ),
  //   InsightTypes::Savings,
  //   None,
  // ))
}
