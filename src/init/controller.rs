use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}
#[post("/connect")]

async fn connect(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}
