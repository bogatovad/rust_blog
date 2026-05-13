mod infrastructure;
mod domain;
mod application;
mod presentation;
mod data;

use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web::{get, web, Responder};
use crate::infrastructure::database::{get_pool, run_migrations};
use envconfig::Envconfig;
use dotenvy::dotenv;

use crate::presentation::http_handlers::configure;
use crate::data::user_repository::UserRepository;


#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "ADDR")]
    pub addr: String,
}


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::init_from_env().unwrap();
    println!("Database URL: {}", config.database_url);

    // Инициализируем логгер
    env_logger::init();

    let pool = get_pool(config.database_url).await;
    let _ = run_migrations(&pool).await;
    let repo = UserRepository::new(Arc::new(pool));


    HttpServer::new(move || {
        let cors = Cors::default()
            //.allowed_origin(&cfg.cors_origin)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(repo.clone()))
            //.app_data(web::Data::new(pool.clone()))
            //.app_data(web::Data::new(cfg.clone()))
            .configure(configure)
    })
        .bind(config.addr)?
        .run()
        .await
}