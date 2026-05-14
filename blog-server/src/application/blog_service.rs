use crate::data::post_repository::PostRepository;
use crate::domain::post::{Post, PostCreate};

#[derive(Clone)]
pub struct PostService{
    repo: PostRepository,
}

impl PostService{
    pub fn new(repo: PostRepository) -> Self{
        Self{ repo }
    }

    pub async fn create_post(&self, post: PostCreate){
        self.repo.create_post(post).await
    }

    pub async fn get_post(&self, id: i64) -> Post{
        self.repo.get_post(id).await
    }

    pub async fn delete_post(&self, id: i64) -> Option<bool>{
        self.repo.delete_post(id).await
    }

    pub async fn get_all_posts(&self) -> Vec<Post>{
        self.repo.get_all_posts().await
    }
}