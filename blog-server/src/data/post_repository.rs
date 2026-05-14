use sqlx::{PgPool};
use crate::domain::post::{Post, PostCreate};
use std::sync::{Arc};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct PostRepository{
    pool: Arc<PgPool>,
}

impl PostRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self{ pool }
    }

    pub async fn create_post(&self, post: PostCreate){
        let author_id = 1;
        sqlx::query(
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(post.title)
        .bind(post.content)
        .bind(author_id)
        .execute(&*self.pool)
        .await.unwrap();
    }

    pub async fn get_post(&self, id: i64) -> Post {
        sqlx::query_as!(
            Post,
            r#"
            SELECT
                id,
                title,
                content,
                author_id,
                created_at,
                updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await.unwrap()
    }

    pub async fn delete_post(&self, id: i64) -> Option<bool> {
        let rows_affected = sqlx::query(
            r#"DELETE FROM posts WHERE id = $1"#)
            .bind(id)
            .execute(&*self.pool)
            .await.unwrap()
            .rows_affected();

        Some(rows_affected > 0)
    }

    pub async fn get_all_posts(&self) -> Vec<Post> {
        sqlx::query_as!(
        Post,
        r#"
        SELECT
            id,
            title,
            content,
            author_id,
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
        FROM posts
        ORDER BY created_at DESC
        "#
        )
            .fetch_all(&*self.pool)
            .await
            .unwrap()
    }
}