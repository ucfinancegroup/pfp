mod controllers;
mod models;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware};
use actix_session::CookieSession;

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
        .secure(true)
      )
      .wrap(middleware::Logger::default())
      .configure(controllers::configure)
      .service(root_route)
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
