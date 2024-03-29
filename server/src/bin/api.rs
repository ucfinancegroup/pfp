extern crate pfp_server;
use pfp_server::*;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root_route() -> impl Responder {
  HttpResponse::Ok().body("hello world")
}

cfg_if::cfg_if! {
  if #[cfg(feature="development")] {
    fn create_cookie() -> CookieSession {
      CookieSession::signed(&[0; 32])
          .name("finch-sid")
          .secure(false)
          .http_only(false)
    }
  } else {
    fn create_cookie() -> CookieSession {
      CookieSession::signed(&[0; 32])
          .name("finch-sid")
          .secure(true)
          .http_only(false)
          .expires_in(60 * 60 * 24 * 30) // 30 days expiration
    }
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  common::finchlog::init_log(module_path!());

  let env: services::secrets::Environment =
    services::secrets::Environment::new().expect("Need good env config");

  let db_service = services::db::DatabaseService::new(
    env.database_url,
    env.database_user,
    env.database_pw,
    env.database_name,
  )
  .await;

  let plaid_client = services::finchplaid::ApiClient {
    client_id: env.plaid_client_id,
    secret: env.plaid_sandbox_secret,
    client_name: "finch".to_string(),
  };

  let user_service = services::users::UserService::new(&db_service).await;
  let session_service = services::sessions::SessionService::new(&db_service).await;
  let fin_product_service = services::financial_products::FinProductService::new(&db_service).await;
  let leaderboard_service = services::leaderboards::LeaderboardService::new(&db_service).await;

  HttpServer::new(move || {
    App::new()
      .wrap(create_cookie())
      .wrap(middleware::Logger::default())
      .data(plaid_client.clone())
      .data(user_service.clone())
      .data(session_service.clone())
      .data(fin_product_service.clone())
      .data(leaderboard_service.clone())
      .configure(controllers::configure)
      .service(root_route)
      .wrap(middleware::Logger::default())
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
