use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegistration{
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin{
    username: String,
    password: String,
}