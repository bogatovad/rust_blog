use actix_web::{get, post, delete, web, HttpResponse, Responder};
use crate::domain::user::UserRegistration;
use crate::domain::post::PostCreate;

use crate::application::auth_service::AuthService;
use crate::application::blog_service::PostService;


#[post("/registration")]
async fn registration_user(
    auth_service: web::Data<AuthService>,
    body: web::Json<UserRegistration>,
) -> impl Responder {
    let data = body.into_inner();
    auth_service.registration_user(data.clone()).await;
    HttpResponse::Ok().json(serde_json::json!(data.clone()))
}

#[post("/posts")]
async fn create_post(
    post_service: web::Data<PostService>,
    body: web::Json<PostCreate>,
) -> impl Responder {
    let data = body.into_inner();
    post_service.create_post(data.clone()).await;
    HttpResponse::Ok().json(serde_json::json!(data.clone()))
}

#[get("/posts/{id}")]
async fn get_post(
    path: web::Path<i64>,
    post_service: web::Data<PostService>,
) -> impl Responder {
    let post_id = path.into_inner();
    let post = post_service.get_post(post_id).await;
    HttpResponse::Ok().json(serde_json::json!(post))
}

#[get("/posts")]
async fn get_all_posts(
    post_service: web::Data<PostService>,
) -> impl Responder {
    let posts = post_service.get_all_posts().await;
    HttpResponse::Ok().json(serde_json::json!(posts))
}

#[delete("/posts/{id}")]
async fn delete_post(
    path: web::Path<i64>,
    post_service: web::Data<PostService>,
) -> impl Responder {
    let post_id = path.into_inner();
    let is_deleted = post_service.delete_post(post_id).await;

    if is_deleted.unwrap() == true {
        HttpResponse::Ok().json(serde_json::json!({"message": "Deleted post"}))
    }
    else{
        HttpResponse::Ok().json(serde_json::Value::Null)
    }
}


pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(registration_user)
        .service(create_post)
        .service(get_post)
        .service(get_all_posts)
        .service(delete_post);
}

