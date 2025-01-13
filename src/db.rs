
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::env;
use dotenvy::dotenv;
use sqlx::migrate::MigrateDatabase;

pub async fn connect_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Check if the database exists, and create it if it doesn't
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        Sqlite::create_database(&db_url).await?;
    } else {
        println!("Database already exists");
    }
    let pool  = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}