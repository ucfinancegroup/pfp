extern crate pfp_server;
use pfp_server::*;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

#[get("/")]
async fn root_route() -> impl Responder {
  HttpResponse::Ok().body("hello world")
}

cfg_if::cfg_if! {
  if #[cfg(feature="development")] {
    fn create_cookie() -> CookieSession {
      CookieSession::signed(&[0; 32])
          .name("finch-sid")
          .path("/")
          .secure(false)
          .http_only(false)
    }
  } else {
    fn create_cookie() -> CookieSession {
      CookieSession::signed(&[0; 32])
          .domain("https://finchapp.eastus.cloudapp.azure.com/")
          .name("finch-sid")
          .path("/")
          .secure(true)
          .http_only(false)
          .expires_in(60 * 60 * 24 * 30) // 30 days expiration
    }
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let env: services::secrets::Environment =
    services::secrets::Environment::new().expect("Need good env config");

  let db = services::db::connect_to_mongo(
    env.database_url,
    env.database_user,
    env.database_pw,
    env.database_name,
  )
  .unwrap();

  let plaid_client = Arc::new(Mutex::new(services::finchplaid::ApiClient {
    client_id: env.plaid_client_id,
    secret: env.plaid_sandbox_secret,
    client_name: "finch".to_string(),
    configuration: plaid::apis::configuration::Configuration::default(),
  }));

  HttpServer::new(move || {
    App::new()
      .wrap(create_cookie())
      .wrap(middleware::Logger::default())
      .data(db.clone())
      .data(plaid_client.clone())
      .configure(controllers::configure)
      .service(root_route)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
