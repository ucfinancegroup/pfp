mod common;
mod controllers;
mod models;
mod services;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[get("/")]
async fn root_route() -> impl Responder {
  HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let db_user = dotenv::var("DATABASE_USER").expect("DATABASE_USER is not set in .env file");
  let db_pw = dotenv::var("DATABASE_PW").expect("DATABASE_PW is not set in .env file");
  let db_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");

  let db = services::db_service::connect_to_mongo(uri, db_user, db_pw, db_name).unwrap();

  HttpServer::new(move || {
    App::new()
      .wrap(
        CookieSession::signed(&[0; 32])
          .domain("https://finchapp.eastus.cloudapp.azure.com/")
          .name("sid")
          .path("/")
          .secure(true),
      )
      .wrap(middleware::Logger::default())
      .data(db.clone())
      .configure(controllers::configure)
      .service(root_route)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
