use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    return Pool::builder().build(connection_manager).expect("Failed to create pool");
}
