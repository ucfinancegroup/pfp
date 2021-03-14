mod products;

pub mod common;
pub mod similar_user;

#[allow(non_snake_case)]
pub mod InsightsService {
  use super::*;
  use crate::common::errors::AppError;
  use crate::models::{
    insight_model::{Insight, InsightTypes},
    user_model::User,
  };
  use crate::services::{db::DatabaseService, finchplaid};
  use async_std::task;
  use chrono::{Duration, Utc};
  use futures::stream::StreamExt;
  use log::{debug, info};
  use std::collections::HashMap;
  use std::time; // y is there chrono and time lol
  use wither::{
    mongodb::{
      bson::bson,
      bson::doc,
      bson::Bson::Null,
      options::{FindOneAndUpdateOptions, ReturnDocument},
    },
    Model,
  };

  pub async fn run_insights_service(
    db_service: &DatabaseService,
    _plaid_client: &finchplaid::ApiClient,
  ) -> Result<(), AppError> {
    loop {
      let mut user = get_user_needing_insight(&db_service).await?;

      let generated_insight = generate_insight(&user, &db_service).await?;

      let last = user
        .insights
        .last_mut()
        .ok_or(AppError::new("No Incomplete insight was inserted"))?;

      *last = generated_insight;

      user
        .save(&db_service.db, None)
        .await
        .map_err(|_| AppError::new("Failed to save user"))?;

      task::sleep(time::Duration::from_secs(30)).await;
    }
  }

  // Tries to get ONE user that is eligible for a new insight.
  // Does an async sleep if no user is found, and then tries again (and again...)
  pub async fn get_user_needing_insight(db_service: &DatabaseService) -> Result<User, AppError> {
    loop {
      let one_day_ago = (Utc::now() - Duration::days(1)).timestamp();

      // get a user that needs new insight
      // based on: all insights over a day old, or none at all
      // TODO(c650) -- make this more complicated
      let user_opt = User::find_one_and_update(
      &db_service.db,
      doc!{
        "$or": vec![
          doc!{"insights": doc!{"$size": 0}},
          doc!{"insights": doc!{"$not" : doc!{"$elemMatch" : doc!{"generation_time" : doc!{"$gte" : one_day_ago }}}}}
          ]
        },
        doc!{"$push": doc!{ "insights" : crate::common::into_bson_document(&Insight::default())}},
        FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build(),
      )
      .await
      .map_err(|_e| AppError::new("Error during find_one_and_update"))?;

      if let Some(user) = user_opt {
        return Ok(user);
      }

      debug!("Got no user.");

      let sleep_time = calculate_wait_time(&db_service).await?;

      info!(
        "Trying to get user for Insight generation in {} seconds...",
        sleep_time
      );

      task::sleep(time::Duration::from_secs(sleep_time as u64)).await;
    }
  }

  // TODO -- this function should eventually decide what type of insight
  // and then delegate to a more specific insight generator.
  pub async fn generate_insight(
    user: &User,
    db_service: &DatabaseService,
  ) -> Result<Insight, AppError> {
    // TODO(c650) -- pick which insight to make.
    log::info!("Generating insight for {}", user.email);

    let latest_of_each_insight_type: HashMap<InsightTypes, i64> = {
      // pre-populate hashmap with each implemented insight type.
      let mut h = HashMap::new();
      h.insert(InsightTypes::ProductRecommendation, -1);
      h.insert(InsightTypes::Savings, -2);

      user
        .insights
        .iter()
        .map(|insight: &Insight| (insight.generation_time, insight.insight_type.clone()))
        .fold(h, |mut hm, (time, insight_type)| {
          hm.insert(
            insight_type.clone(),
            hm.get(&insight_type)
              .map_or_else(|| time, |t| (*t).max(time)),
          );
          hm
        })
    };

    // get least recently generated insight type.
    let (insight_type, _) = latest_of_each_insight_type
      .into_iter()
      // prevent us thinking we want Incomplete insight.
      .filter(|(insight_type, _)| *insight_type != InsightTypes::Incomplete)
      .fold(
        (InsightTypes::ProductRecommendation, Utc::now().timestamp()),
        |(ty, ti), (k, v)| {
          if v < ti {
            (k, v)
          } else {
            (ty, ti)
          }
        },
      );

    // TODO(c650) -- alternate between least recent and random

    // finally generate least recent insight type
    match insight_type {
      InsightTypes::ProductRecommendation => {
        products::generate_product_insight(user, db_service).await
      }
      // default to savings,spending, or income
      user_metric => {
        similar_user::generate_similar_user_insight(user, db_service, user_metric).await
      }
    }
  }

  // Determines how long to wait before checking again to see
  // if any user is eligible for new insight
  pub async fn calculate_wait_time(db_service: &DatabaseService) -> Result<i64, AppError> {
    let mut agg = User::collection(&db_service.db)
      .aggregate(
        vec![
          doc! {
            "$project": {
              "last_insight": {
                "$arrayElemAt": bson!([
                  "$insights",
                  -1
                ])
              }
            }
          },
          doc! {
            "$group": {
              "_id": Null,
              "earliest_last_insight_time": {
                "$min": "$last_insight.generation_time"
              }
            }
          },
        ],
        None,
      )
      .await
      .map_err(|_| AppError::new("Error during aggregation"))?;

    let min_time: i64 = agg
      .next()
      .await
      .ok_or(AppError::new("Bad aggregation"))?
      .map_err(|_| AppError::new("Db Error"))
      .and_then(|doc| {
        doc
          .get_i64("earliest_last_insight_time")
          .map_err(|_| AppError::new("no i64 field earliest_last_insight_time"))
      })
      .unwrap_or(Utc::now().timestamp());

    // max with 30s just in case it's somehow otherwise negative sleep_time
    // but that shouldn't happen because if at least one user hasnt had a new insight
    // in over a day then we wouldnt reach this code...
    let sleep_time =
      (Duration::days(1).num_seconds() - (Utc::now().timestamp() - min_time)).max(30);

    Ok(sleep_time)
  }
}

#[cfg(test)]
mod tests {}
