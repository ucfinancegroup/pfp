#[derive(Debug)]
struct Ranking {
  pub leaderboard_type: String,
  pub percentile: f64,
  pub description: String,
}

impl Ranking {
  fn new(leaderboard_type: String, percentile: f64, description: String) -> Ranking {
    Ranking {
      leaderboard_type: percentile.to_string(),
      percentile: percentile,
      description: description.to_string(),
    }
  }
}
