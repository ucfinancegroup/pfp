mod common;
mod controllers;
mod models;
mod services;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
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
  dotenv().ok();
  let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let db_user = dotenv::var("DATABASE_USER").expect("DATABASE_USER is not set in .env file");
  let db_pw = dotenv::var("DATABASE_PW").expect("DATABASE_PW is not set in .env file");
  let db_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");

  let db = services::db::connect_to_mongo(uri, db_user, db_pw, db_name).unwrap();

  let plaid_client = Arc::new(Mutex::new(services::finchplaid::ApiClient {
    client_id: dotenv::var("PLAID_CLIENT_ID").expect("need plaid client id"),
    secret: dotenv::var("PLAID_SANDBOX_SECRET").expect("need plaid sandbox secret"),
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
