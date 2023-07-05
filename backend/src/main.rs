#![allow(dead_code, unused_imports)]
use actix_web::{web, App, HttpServer};
mod state;
mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (state, host, port, io_mode) = state::State::init()?;
    let app_state = web::Data::new(state);

    if io_mode {
        todo!("Implement IO mode")
    } else {
        HttpServer::new(move || App::new().app_data(app_state.clone()))
            .bind((host, port))?
            .run()
            .await
    }
}
