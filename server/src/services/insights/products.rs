use crate::common::errors::AppError;
use crate::models::{
  financial_product_model::FinancialProduct,
  insight_model::{Insight, InsightTypes},
  user_model::User,
};
use crate::services::{db::DatabaseService, insights::common::match_income_range};
use futures::stream::{Stream, StreamExt};
use rand::seq::SliceRandom;
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
  log::info!("generating a product insight for {}", user.email.clone());

  let agg = User::collection(&db_service.db)
    .aggregate(
      vec![match_income_range(&user), project_account_records()],
      None,
    )
    .await
    .map_err(|_| AppError::new("Error during aggregation"))?;

  // extract accounts from aggregation
  let extracted_known_accounts = agg.map(extract_known_account_ids);

  // compute frequency of each account (by id)
  let known_account_frequencies = get_frequency_of_known_accounts(extracted_known_accounts).await;

  // get most frequent one
  let most_frequent =
    get_most_frequent_account(known_account_frequencies).ok_or(AppError::new("No accounts"))?;

  // try to find the most frequent one by id
  let fp = FinancialProduct::find_one(&db_service.db, doc! {"_id" : most_frequent.clone()}, None)
    .await
    .map_err(|_| AppError::new("could not resolve financial product"))?
    .ok_or(AppError::new("Could not deserialise financial product"))?;

  log::info!(
    "Recommending {} ({:?}) to user {}",
    &fp.name,
    fp.id(),
    user.email.clone()
  );

  Ok(Insight::new(
    "Try this new account others like you are using".to_string(),
    format!(
      "Many users similar to you are using a {} account: {}",
      fp.name, fp.description
    ),
    InsightTypes::ProductRecommendation,
    fp.image_url,
  ))
}

async fn get_frequency_of_known_accounts<
  S: StreamExt + Stream<Item = Result<Vec<ObjectId>, AppError>>,
>(
  extracted_known_accounts: S,
) -> HashMap<ObjectId, i32> {
  extracted_known_accounts
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
    .await
}

// collects accounts and considers the most frequent
// randomly picks which one to consider the "most" frequent
fn get_most_frequent_account(
  known_account_frequencies: HashMap<ObjectId, i32>,
) -> Option<ObjectId> {
  let largest_freq = known_account_frequencies
    .iter()
    .map(|(_, v)| v)
    .cloned()
    .fold(-1i32, i32::max);

  let mut rng = rand::thread_rng();

  known_account_frequencies
    .iter()
    .filter(|(_, v)| **v == largest_freq)
    .map(|(k, _)| k)
    .cloned()
    .collect::<Vec<_>>()
    .choose(&mut rng)
    .cloned()
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

#[cfg(test)]
mod tests {
  use super::*;
  use actix_rt;

  #[actix_rt::test]
  async fn test_calc_freq_of_known_accounts() {
    let object_ids: Vec<ObjectId> = (1..10).map(|_| ObjectId::new()).collect();

    let extracted_known_accounts: Vec<Vec<ObjectId>> = vec![
      vec![object_ids[0].clone(), object_ids[1].clone()],
      vec![object_ids[0].clone(), object_ids[2].clone()],
      vec![object_ids[1].clone(), object_ids[2].clone()],
      vec![object_ids[0].clone()],
    ];

    let known_account_frequencies = get_frequency_of_known_accounts(
      futures::stream::iter(extracted_known_accounts).map(|e| Ok(e)),
    )
    .await;

    let mut freqs = known_account_frequencies
      .clone()
      .into_iter()
      .collect::<Vec<(ObjectId, i32)>>();

    let mut ans = vec![
      (object_ids[0].clone(), 3),
      (object_ids[1].clone(), 2),
      (object_ids[2].clone(), 2),
    ];

    freqs.sort();
    ans.sort();

    // check that frequencies are correct
    assert_eq!(freqs, ans);

    // check that most frequent is returned
    assert_eq!(
      get_most_frequent_account(known_account_frequencies),
      Some(object_ids[0].clone())
    );
  }
}
