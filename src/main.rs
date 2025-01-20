use api::update_word;
use axum::{
    routing::{get, post, put, Router},
    Extension,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net;

mod api;
mod db;
mod model;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = match db::connect_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to the database: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/items", post(api::create_item))
        .route("/items/{id}", put(api::update_item))
        .route("/items", get(api::get_items))
        .route("/words", get(api::get_words))
        .route("/word/{:id}", put(update_word))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
