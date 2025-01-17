
use sqlx::{Pool, Sqlite};
use std::env;
use dotenvy::dotenv;
use sqlx::migrate::MigrateDatabase;
use sqlx::SqlitePool;


pub async fn connect_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();
    
    // Check if the database exists, and create it if it doesn't
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        Sqlite::create_database(&db_url).await?;

        // If the database is newly created, run the migrations
        let pool = SqlitePool::connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(pool)
    } else {
        println!("Database already exists");
        
        // If the database already exists, just connect to it without running migrations
        let pool = SqlitePool::connect(&db_url).await?;
        Ok(pool)
    }
}
