use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

struct State {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<State>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {app_name} called {counter} time!"))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}
#[post("/connect")]

async fn connect(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Step 1: Get all the files in the /home/{user}/.benday/ directory
    // Step 2: Load a global file that contains the list of projects (and their paths)
    // Step 3: Wait for the client (frontend) to send the connection request

    let state = web::Data::new(State {
        counter: Mutex::new(0),
        app_name: String::from("Actix Web"),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hello)
            .service(connect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
