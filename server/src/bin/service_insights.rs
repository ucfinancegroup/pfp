extern crate pfp_server;
use actix_web;
use pfp_server::*;
use services::{db::DatabaseService, finchplaid, insights::InsightsService, secrets::Environment};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  simple_logger::SimpleLogger::new()
    .with_level(log::LevelFilter::Off)
    .with_module_level("pfp_server", log::LevelFilter::Info)
    .with_module_level(module_path!(), log::LevelFilter::Info)
    .init()
    .unwrap();

  let env = Environment::new().expect("Need good env config");

  let db_service = DatabaseService::new(
    env.database_url,
    env.database_user,
    env.database_pw,
    env.database_name,
  )
  .await;

  let plaid_client = finchplaid::ApiClient {
    client_id: env.plaid_client_id,
    secret: env.plaid_sandbox_secret,
    client_name: "finch".to_string(),
    configuration: plaid::apis::configuration::Configuration::default(),
  };

  InsightsService::run_insights_service(db_service, plaid_client)
    .await
    .unwrap();

  Ok(())
}
