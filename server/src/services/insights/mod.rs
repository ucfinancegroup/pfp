#[allow(non_snake_case)]
pub mod InsightsService {
  use crate::services::{db::DatabaseService, finchplaid};
  use log::{debug, info, warn};

  pub async fn run_insights_service(
    db_service: DatabaseService,
    plaid_client: finchplaid::ApiClient,
  ) -> std::io::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_reee() {}
}
