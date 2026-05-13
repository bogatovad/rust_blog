use actix_web::{get, post, web, HttpResponse, Responder};
use crate::domain::user::UserRegistration;
use crate::data::user_repository::UserRepository;

#[post("/registration")]
async fn registration_user(
    repo: web::Data<UserRepository>,
    body: web::Json<UserRegistration>,
) -> impl Responder {
    let data = body.into_inner();
    repo.registration_user(data.clone()).await;
    HttpResponse::Ok().json(serde_json::json!(data.clone()))
}
pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(registration_user);
}

