use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Post{
    id: i64,
    title: String,
    content: String,
    author_id: i64,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct PostCreate{
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostUpdate{
    title: String,
    content: String,
}