#![allow(dead_code, unused_imports)]
use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod state;
mod update;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Run in development mode
    #[arg(short, long)]
    dev: bool,

    #[arg(short, long, default_value = "8080")]
    port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

fn main_dev() {
    // update::update_request::test_to_json();
    update::update_response::test_to_json();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.dev {
        main_dev();
        return Ok(());
    }

    let state = web::Data::new(state::State {
        ast: Mutex::new(vec![]),
        project_path: Mutex::new(std::path::PathBuf::new()),
    });
    HttpServer::new(move || App::new().app_data(state.clone()))
        .bind((args.host, args.port))?
        .run()
        .await
}
