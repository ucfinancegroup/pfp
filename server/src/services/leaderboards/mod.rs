use crate::common::errors::ApiError;
use crate::models::leaderboard_model::BoardTypes;
use crate::models::leaderboard_model::Ranking;
use crate::models::user_model::User;
use crate::services::db;
use crate::services::db::DatabaseService;
use crate::services::insights::similar_user;
use chrono::{Duration, Utc};
use wither::Model;

#[derive(Clone)]
pub struct LeaderboardService {
  db: DatabaseService,
}

#[allow(non_snake_case)]
impl LeaderboardService {
  pub async fn new(db: &db::DatabaseService) -> LeaderboardService {
    LeaderboardService { db: db.clone() }
  }

  pub async fn get_ranking(&self, board: BoardTypes, mut user: User) -> Result<Ranking, ApiError> {
    let one_day_ago = (Utc::now() - Duration::days(1)).timestamp();

    let index = board as usize;

    println!("{}", index);

    if index >= user.rankings.len() {
      user.rankings.resize(index + 1, Ranking::default());
    }

    if user.rankings[index].generation_time > one_day_ago {
      return Ok((&user.rankings[index]).clone());
    }

    let rank = similar_user::generate_ranking(user.clone(), &self.db, board)
      .await
      .map_err(|err| err.into())?;
    user.rankings[index] = rank.clone();
    user.save(&self.db.db, None).await.map_or_else(
      |_| Err(ApiError::new(500, "Database Error".to_string())),
      |_| Ok(rank),
    )
  }
}
