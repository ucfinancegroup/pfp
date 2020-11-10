mod common;
mod controllers;
mod models;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{bson::doc, Client};

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

  let connection_str = format!(
    "mongodb+srv://{}:{}@{}/{}?w=majority",
    db_user, db_pw, uri, db_name
  );

  let client = Client::with_uri_str(&connection_str)
    .await
    .expect("Failed to initialize client.");

  let db = client.database(&db_name);

  db.run_command(doc! {"ping": 1}, None)
    .await
    .expect("Failed to ping client");

  println!("Connected successfully.");

  for coll_name in db
    .list_collection_names(None)
    .await
    .expect("Failed to print collections.")
  {
    println!("collection: {}", coll_name);
  }

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
