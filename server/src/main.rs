mod controllers;
mod models;

#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root_route() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root_route)
            .configure(controllers::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
