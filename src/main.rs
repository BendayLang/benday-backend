#![allow(dead_code, unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod init;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Step 3: Wait for the client (frontend) to send the connection request

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
