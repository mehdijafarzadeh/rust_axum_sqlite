use axum::{
    extract::{Extension},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Sqlite};
use crate::model::{Item, NewItem};


pub async fn get_items(Extension(pool): Extension<Pool<Sqlite>>) -> Json<Vec<Item>> {
    let items = sqlx::query_as::<_, Item>("SELECT id, name FROM items")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch items.");

    Json(items)
}

pub async fn create_item(
    Extension(pool): Extension<Pool<Sqlite>>,
    Json(new_item): Json<NewItem>
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
        .bind(&new_item.name)
        .execute(&pool)
        .await;

    match result {
        Ok(inserted) => {
            let id = inserted.last_insert_rowid();
            let created_item = Item {
                id: id as i32,
                name: new_item.name.clone(),
            };

            Ok((StatusCode::CREATED, Json(created_item)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error inserting item: {}", e))),
    }
}
