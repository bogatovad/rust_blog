use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Post{
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostCreate{
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostUpdate{
    pub title: String,
    pub content: String,
}