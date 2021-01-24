use crate::common::errors::AppError;
use crate::models::{
  insight_model::{Insight, InsightTypes},
  user_model::{Snapshot, User},
};
use crate::services::db::DatabaseService;
use chrono::{DateTime, Utc};
use futures::stream::{Stream, StreamExt};
use wither::{
  mongodb::bson::{bson, doc, Bson},
  Model,
};

pub fn match_income_range(u: &User) -> bson::Document {
  doc! {
    "$match": {
      "income": {
        "$gte": u.income * 0.9,
        "$lte": u.income * 1.10
      },
      "snapshots": {
        "$not": {
          "$size": 0
        }
      },
      "_id": {
        "$ne": u.id().map_or_else(|| Bson::Null, |id| bson!(id))
      }
    }
  }
}

pub fn project_snapshots(since: DateTime<Utc>) -> bson::Document {
  doc! {
    "$project": {
      "snapshots": {
        "$filter": {
          "input": "$snapshots",
          "cond": {
            "$gte": ["$$this.snapshot_time", since.timestamp()]
          }
        }
      }
    }
  }
}

pub async fn generate_similar_user_insight(
  user: &User,
  db_service: &DatabaseService,
) -> Result<Insight, AppError> {
  let lookback = chrono::Duration::days(30);
  let since = Utc::now() - lookback;

  let agg = User::collection(&db_service.db)
    .aggregate(
      vec![match_income_range(&user), project_snapshots(since)],
      None,
    )
    .await
    .map_err(|_| AppError::new("Error during aggregation"))?;

  let extracted_snapshots = agg.map(extract_snapshots);

  let (less, total): (i32, i32) =
    compare_snapshots_to_user(&user.snapshots, extracted_snapshots, &since).await;

  if total <= 0 {
    return Err(AppError::new("No peers for insight generation"));
  }

  Ok(Insight::new(
    "Savings Insight".to_string(),
    format!(
      "Your savings over the last {} days puts you above {}% of similar users!",
      lookback.num_days(),
      100 * less / total
    ),
    InsightTypes::Savings,
    None,
  ))
}

fn extract_snapshots(
  snaps: Result<bson::Document, wither::mongodb::error::Error>,
) -> Result<Vec<Snapshot>, AppError> {
  let doc = snaps.map_err(|_| AppError::new("Aggregation Error"))?;

  let snapshots_doc = doc
    .get("snapshots")
    .ok_or(AppError::new("No snapshots field"))
    .map(|s| s.clone())?;

  bson::from_bson::<Vec<Snapshot>>(snapshots_doc)
    .map_err(|_| AppError::new("Deserialisation error"))
}

// returns (less, total)
async fn compare_snapshots_to_user<
  S: StreamExt + Stream<Item = Result<Vec<Snapshot>, AppError>>,
>(
  users_snapshots: &Vec<Snapshot>,
  similar_users_snapshots: S,
  since: &DateTime<Utc>,
) -> (i32, i32) {
  let my_rate = calculate_rates(users_snapshots, since);

  let res: (i32, i32) = similar_users_snapshots
    .fold((0, 0), |(less, total), snaps| async move {
      snaps.map_or_else(
        |_| (less, total),
        |snaps| {
          let rate = calculate_rates(&snaps, since);

          let new_less = less + if rate < my_rate { 1 } else { 0 };

          (new_less, total + 1)
        },
      )
    })
    .await;

  res
}

fn calculate_rates(snapshots: &Vec<Snapshot>, since: &DateTime<Utc>) -> f64 {
  let tstamp = since.timestamp();
  let filtered = snapshots
    .iter()
    .filter(|s| s.snapshot_time >= tstamp)
    .collect::<Vec<&Snapshot>>();

  let f = filtered.first();
  let l = filtered.last();

  let rate = match (f, l) {
    (Some(first), Some(last)) => {
      if first.snapshot_time != last.snapshot_time {
        let diff: f64 = (last.running_savings.amount - first.running_savings.amount) as f64;
        diff / (last.snapshot_time - first.snapshot_time) as f64
      } else {
        0.0
      }
    }
    _ => 0.0,
  };

  rate
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::Money;
  use actix_rt;
  use chrono::NaiveDateTime;

  #[actix_rt::test]
  async fn test_compare_snapshots_to_user() {
    let other_users_snapshots: Vec<Result<Vec<Snapshot>, AppError>> = vec![
      Ok(vec![
        Snapshot {
          net_worth: Money { amount: 0 },
          running_savings: Money { amount: 0 },
          running_spending: Money { amount: 0 },
          running_income: Money { amount: 0 },
          snapshot_time: 50,
        },
        Snapshot {
          net_worth: Money { amount: 0 },
          running_savings: Money { amount: 5000 },
          running_spending: Money { amount: 0 },
          running_income: Money { amount: 0 },
          snapshot_time: 100,
        },
      ]),
      Ok(vec![
        Snapshot {
          net_worth: Money { amount: 0 },
          running_savings: Money { amount: 0 },
          running_spending: Money { amount: 0 },
          running_income: Money { amount: 0 },
          snapshot_time: 50,
        },
        Snapshot {
          net_worth: Money { amount: 0 },
          running_savings: Money { amount: 4500 },
          running_spending: Money { amount: 0 },
          running_income: Money { amount: 0 },
          snapshot_time: 100,
        },
      ]),
    ];

    let this_users_snapshots: Vec<Snapshot> = vec![
      Snapshot {
        net_worth: Money { amount: 0 },
        running_savings: Money { amount: 0 },
        running_spending: Money { amount: 0 },
        running_income: Money { amount: 0 },
        snapshot_time: 50,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_savings: Money { amount: 4700 },
        running_spending: Money { amount: 0 },
        running_income: Money { amount: 0 },
        snapshot_time: 100,
      },
    ];

    let (less, total): (i32, i32) = compare_snapshots_to_user(
      &this_users_snapshots,
      futures::stream::iter(other_users_snapshots.into_iter()),
      &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1, 0), Utc),
    )
    .await;

    assert_eq!(less, 1);
    assert_eq!(total, 2);
  }
}
