use axum::{
    Extension,
    routing::{Router, get, post},
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
        .route("/api/items", post(api::create_item))
        .route("/api/items", get(api::get_items))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
