use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::{ Pool, PooledConnection };

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    return Pool::builder().build(connection_manager).expect("Failed to create pool");
}

pub fn get_connection(pool: &ConnectionPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    return pool.get().expect("Failed to get connection from pool");
}
