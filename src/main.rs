use dotenvy::dotenv;
use std::{ env, sync::Arc };
use actix_web::{ get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder };
use log::info;

pub mod database;
pub mod routes;
pub mod auth;
pub mod utils;
pub mod errors;

pub struct AppContext {
    pool: database::connection::ConnectionPool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::auth::init_routes);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    info!("Starting server...");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let pool = database::connection::create_pool();
    let ctx = Arc::new(AppContext { pool });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(ctx.clone()))
            .service(hello)
            .configure(init_routes)
    })
        .bind(format!("{}:{}", host, port))?
        .run();

    info!("Server running at http://{}:{}", host, port);

    server.await
}
