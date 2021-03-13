use crate::common::errors::AppError;
use crate::models::{
  leaderboard_model::{BoardType, Ranking},
  user_model::{Snapshot, User},
};
use crate::services::insights::common::match_income_range;
use chrono::{DateTime, Utc};
use futures::stream::{Stream, StreamExt};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use wither::mongodb::bson::doc;
use wither::mongodb::Database;

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
  // let agg = db
  //   .aggregate(
  //     vec![match_income_range(&user), project_snapshots(since)],
  //     None,
  //   )
  //   .await
  //   .map_err(|_| AppError::new("Error during aggregation"))?;

  // let extracted_snapshots = agg.map(extract_snapshots);

  // let metrics: SimilarUserMetrics =
  //   compare_snapshots_to_user(&user.snapshots, extracted_snapshots, &since).await;

  return Ok(SimilarUserMetrics::default());
}

pub async fn generate_ranking(
  user: &User,
  db: &Database,
  board_type: BoardType,
) -> Result<Ranking, AppError> {
  log::info!(
    "generating a similar user insight for {}",
    user.email.clone()
  );
  let lookback = chrono::Duration::days(30);
  let since = Utc::now() - lookback;

  let metrics = generate_metric(user, db, since).await?;
  // if metrics.total_similar_users <= 0 {
  //   return Err(AppError::new("No peers for insight generation"));
  // }
  Ok(Ranking {
    leaderboard_type: board_type,
    percentile: 19.1,
    description: "Test".to_string(),
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
