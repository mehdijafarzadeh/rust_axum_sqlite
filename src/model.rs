use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewItem {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateItem {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub definition: String,
}
// #[derive(Deserialize)]
// pub struct NewWord {
//     pub word: String,
//     pub definition: String,
// }
#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateWord {
    pub word: String,
    pub definition: String,
}
