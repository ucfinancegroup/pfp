mod common;
mod controllers;
mod models;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root_route() -> impl Responder {
  HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(
        CookieSession::signed(&[0; 32])
          .domain("https://finchapp.eastus.cloudapp.azure.com/")
          .name("sid")
          .path("/")
          .secure(true),
      )
      .wrap(middleware::Logger::default())
      .configure(controllers::configure)
      .service(root_route)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
