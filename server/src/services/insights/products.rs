use crate::common::errors::AppError;
use crate::models::{
  insight_model::{Insight, InsightTypes},
  user_model::User,
};
use crate::services::{db::DatabaseService, insights::common::match_income_range};
use futures::stream::{Stream, StreamExt};
use std::collections::HashMap;
use wither::{
  mongodb::bson::{self, doc, oid::ObjectId},
  Model,
};

pub fn project_account_records() -> bson::Document {
  doc! {
      "$project": {
        "known_account_ids": {
          "$map" : {
            "input": "$account_records",
            "as": "account_record",
            "in" : "$$account_record.known_account_id"

          }
        }
      }
  }
}

pub async fn generate_product_insight(
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

  let extracted_known_accounts = agg.map(extract_known_account_ids);

  let hm = extracted_known_accounts
    .fold(
      HashMap::new(),
      |mut acc: HashMap<ObjectId, i32>, known_account_ids| {
        let _ = known_account_ids.and_then(|ids| {
          ids.into_iter().for_each(|id| {
            let val = acc.get(&id).or(Some(&0)).unwrap() + 1;
            acc.insert(id, val);
          });
          Ok(())
        });

        futures::future::ready(acc)
      },
    )
    .await;

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

fn extract_known_account_ids(
  known_account_ids: Result<bson::Document, wither::mongodb::error::Error>,
) -> Result<Vec<ObjectId>, AppError> {
  let doc = known_account_ids.map_err(|_| AppError::new("Aggregation Error"))?;

  let known_account_ids = doc
    .get("known_account_ids")
    .ok_or(AppError::new("No known_account_ids field"))
    .map(|s| s.clone())?;

  bson::from_bson::<Vec<ObjectId>>(known_account_ids)
    .map_err(|_| AppError::new("Deserialisation error"))
}
