use diesel::{ Connection, PgConnection };
use dotenvy::dotenv;
use std::env;
use actix_web::{ get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder };
use log::info;

pub mod database;
pub mod routes;
pub mod auth;

pub fn create_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url).expect(
        &format!("Error connecting to {}", database_url)
    );
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    info!("Starting server...");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let pool = database::connection::get_pool();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool.clone())
            .service(hello)
            .route("/users", web::get().to(routes::auth::get_users))
    })
        .bind(format!("{}:{}", host, port))?
        .run();

    info!("Server running at http://{}:{}", host, port);

    server.await
}
