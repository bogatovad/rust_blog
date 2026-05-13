use crate::domain::user::{User, UserRegistration, UserLogin};

use std::sync::Arc;
use sqlx::{Executor, PgPool};

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self{ pool }
    }
    pub async fn registration_user(&self, user: UserRegistration) {
        let password_hash = user.password;

        sqlx::query(r#"INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)"#)
            .bind(user.username)
            .bind(user.email)
            .bind(password_hash)
            .execute(&*self.pool)
            .await.unwrap();
    }
}