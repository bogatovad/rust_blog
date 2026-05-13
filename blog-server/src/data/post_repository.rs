use sqlx::{PgPool, migrate, };
use crate::domain::post::{Post, PostCreate, PostUpdate};
use std::sync::{Arc, Mutex};


struct PostRepository{
    pool: Arc<Mutex<PgPool>>,
}

// impl PostRepository {
//     fn new(pool: Arc<Mutex<PgPool>>) -> Self {
//         Self{ pool }
//     }
//
//     fn create_post(&self, post: PostCreate) -> Post {
//         let guard = self.pool.lock().unwrap();
//
//
//     }
// }