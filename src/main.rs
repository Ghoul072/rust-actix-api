use diesel::{ Connection, PgConnection };
use dotenvy::dotenv;
use std::env;
use actix_web::{ get, App, HttpResponse, HttpServer, Responder };
use log::info;

pub mod database;

pub fn create_connection() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let pool = database::connection::get_pool();
    let server = HttpServer::new(move || { App::new().app_data(pool.clone()).service(hello) })
        .bind(format!("{}:{}", host, port))?
        .run();

    info!("Server running at http://{}:{}", host, port);

    server.await
}
