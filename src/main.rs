#![allow(dead_code, unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod init;
mod state;
mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or("".to_string());
    if arg == "--dev" {
        update::test_to_json();
        return Ok(());
    }

    let state = web::Data::new(state::State {});
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(init::controller::connect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
