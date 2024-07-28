use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().expect(".env file not found");

    let database_url = env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    println!("Database URL: {}", database_url);

    let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");
    println!("Migrations Directory: {}", migrations_dir);
}
