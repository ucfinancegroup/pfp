mod common;
mod controllers;
mod models;

use actix_session::CookieSession;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, Client};
use dotenv::dotenv;

#[get("/")]
async fn root_route() -> impl Responder {
  HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let dbUser = dotenv::var("DATABASE_USER").expect("DATABASE_USER is not set in .env file");
  let dbPW = dotenv::var("DATABASE_PW").expect("DATABASE_PW is not set in .env file");
  let dbName = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");

  let client = Client::with_uri_str(&format!("mongodb+srv://{}:{}@{}/{}?w=majority", dbUser,  dbPW,  uri, dbName)).await.expect("Failed to initialize client.");
  let db = client.database("Finch");

  db.run_command(doc! {"ping": 1}, None).await.expect("Failed to ping client");
  println!("Connected successfully.");

  for coll_name in db.list_collection_names(None).await.expect("Failed to print collections."){
    println!("collection: {}", coll_name);
  }

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
