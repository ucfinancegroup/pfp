use crate::common::errors::AppError;
use crate::models::{
  leaderboard_model::{BoardTypes, Ranking},
  user_model::{Snapshot, User},
};

use crate::services::leaderboards::Database;
use chrono::{DateTime, Utc};
use futures::stream::{Stream, StreamExt};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use wither::{mongodb::bson::doc, Model};

#[derive(Clone, Copy)]
struct SimilarUserMetrics {
  pub spending_less: i64,
  pub savings_less: i64,
  pub income_less: i64,
  pub total_similar_users: i64,
}

impl Default for SimilarUserMetrics {
  fn default() -> Self {
    SimilarUserMetrics {
      spending_less: 0,
      savings_less: 0,
      income_less: 0,
      total_similar_users: 0,
    }
  }
}

impl std::ops::Add for SimilarUserMetrics {
  type Output = SimilarUserMetrics;

  fn add(self, other: Self) -> Self::Output {
    SimilarUserMetrics {
      spending_less: self.spending_less + other.spending_less,
      savings_less: self.savings_less + other.savings_less,
      income_less: self.income_less + other.income_less,
      total_similar_users: self.total_similar_users + other.total_similar_users,
    }
  }
}

#[derive(Clone)]
struct UserMetricRates {
  pub spending_rate: Decimal,
  pub savings_rate: Decimal,
  pub income_rate: Decimal,
}

impl UserMetricRates {
  pub fn new(start: &Snapshot, end: &Snapshot) -> Self {
    if start.snapshot_time == end.snapshot_time {
      return Self::default();
    }

    let elapsed = Decimal::new(end.snapshot_time - start.snapshot_time, 0);

    UserMetricRates {
      spending_rate: (end.running_spending - start.running_spending).amount / elapsed,
      savings_rate: (end.running_savings - start.running_savings).amount / elapsed,
      income_rate: (end.running_income - start.running_income).amount / elapsed,
    }
  }

  pub fn compare(&self, other: &Self) -> SimilarUserMetrics {
    SimilarUserMetrics {
      spending_less: (self.spending_rate > other.spending_rate) as i64,
      savings_less: (self.savings_rate > other.savings_rate) as i64,
      income_less: (self.income_rate > other.income_rate) as i64,
      total_similar_users: 1,
    }
  }
}

impl Default for UserMetricRates {
  fn default() -> Self {
    UserMetricRates {
      spending_rate: dec!(0),
      savings_rate: dec!(0),
      income_rate: dec!(0),
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

async fn generate_metric(
  user: &User,
  db: &Database,
  since: DateTime<Utc>,
) -> Result<SimilarUserMetrics, AppError> {
  let agg = User::collection(&db)
    .aggregate(
      vec![project_snapshots(since)],
      None,
    )
    .await
    .map_err(|_| AppError::new("Error during aggregation"))?;

  let extracted_snapshots = agg.map(extract_snapshots);

  let metrics: SimilarUserMetrics =
    compare_snapshots_to_user(&user.snapshots, extracted_snapshots, &since).await;

  return Ok(metrics);
}

pub async fn generate_ranking(
  user: &User,
  db: &Database,
  board: String,
) -> Result<Ranking, AppError> {
  log::info!("get ranking for {}", user.email.clone());

  let lookback = chrono::Duration::days(365);
  let since = Utc::now() - lookback;
  let metrics = generate_metric(user, db, since).await?;

  Ok(if board.to_lowercase() == "savings" {
    Ranking {
      leaderboard_type: BoardTypes::Savings,
      percentile: 100.0 * metrics.savings_less as f64 / metrics.total_similar_users as f64,
      description: "Savings Leaderboard".to_string(),
    }
  } else if board.to_lowercase() == "spending" {
    Ranking {
      leaderboard_type: BoardTypes::Spending,
      percentile: 100.0 * metrics.spending_less as f64 / metrics.total_similar_users as f64,
      description: "Spending Leaderboard".to_string(),
    }
  } else {
    Ranking {
      leaderboard_type: BoardTypes::Income,
      percentile: 100.0 * metrics.income_less as f64 / metrics.total_similar_users as f64,
      description: "Income Leaderboard".to_string(),
    }
  })
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
) -> SimilarUserMetrics {
  let my_rate = calculate_rates(users_snapshots, since);

  let res: SimilarUserMetrics = similar_users_snapshots
    .fold(SimilarUserMetrics::default(), |metrics, snaps| {
      futures::future::ready(snaps.map_or_else(
        |_| metrics,
        |snaps| {
          let rate = calculate_rates(&snaps, since);
          metrics + my_rate.compare(&rate)
        },
      ))
    })
    .await;

  res
}

fn calculate_rates(snapshots: &Vec<Snapshot>, since: &DateTime<Utc>) -> UserMetricRates {
  let tstamp = since.timestamp();
  let filtered = snapshots
    .iter()
    .filter(|s| s.snapshot_time >= tstamp)
    .collect::<Vec<&Snapshot>>();

  let f = filtered.first();
  let l = filtered.last();

  match (f, l) {
    (Some(first), Some(last)) => UserMetricRates::new(&first, &last),
    _ => UserMetricRates::default(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::Money;
  use actix_rt;
  use chrono::NaiveDateTime;
  use rust_decimal_macros::dec;

  #[actix_rt::test]
  async fn test_compare_snapshots_to_user() {
    let other_users_snapshots: Vec<Result<Vec<Snapshot>, AppError>> = vec![
      Ok(vec![
        Snapshot {
          net_worth: Money::new(dec!(0)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(0)),
          running_income: Money::new(dec!(0)),
          snapshot_time: 50,
        },
        Snapshot {
          net_worth: Money::new(dec!(0)),
          running_savings: Money::new(dec!(5000)),
          running_spending: Money::new(dec!(0)),
          running_income: Money::new(dec!(0)),
          snapshot_time: 100,
        },
      ]),
      Ok(vec![
        Snapshot {
          net_worth: Money::new(dec!(0)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(0)),
          running_income: Money::new(dec!(0)),
          snapshot_time: 50,
        },
        Snapshot {
          net_worth: Money::new(dec!(0)),
          running_savings: Money::new(dec!(4500)),
          running_spending: Money::new(dec!(0)),
          running_income: Money::new(dec!(0)),
          snapshot_time: 100,
        },
      ]),
    ];

    let this_users_snapshots: Vec<Snapshot> = vec![
      Snapshot {
        net_worth: Money::new(dec!(0)),
        running_savings: Money::new(dec!(0)),
        running_spending: Money::new(dec!(0)),
        running_income: Money::new(dec!(0)),
        snapshot_time: 50,
      },
      Snapshot {
        net_worth: Money::new(dec!(0)),
        running_savings: Money::new(dec!(4700)),
        running_spending: Money::new(dec!(0)),
        running_income: Money::new(dec!(0)),
        snapshot_time: 100,
      },
    ];

    let metrics: SimilarUserMetrics = compare_snapshots_to_user(
      &this_users_snapshots,
      futures::stream::iter(other_users_snapshots.into_iter()),
      &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1, 0), Utc),
    )
    .await;

    assert_eq!(metrics.savings_less, 1);
    assert_eq!(metrics.total_similar_users, 2);
  }
}
