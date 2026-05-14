use crate::data::user_repository::UserRepository;
use crate::domain::user::UserRegistration;

#[derive(Clone)]
pub struct AuthService{
    repo: UserRepository
}

impl AuthService{
    pub fn new(repo: UserRepository) -> Self{
        Self { repo }
    }

    pub async fn registration_user(&self, user: UserRegistration){
        self.repo.registration_user(user).await
    }
}