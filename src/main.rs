use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

mod controller;
mod service;
mod repository;
mod models;
mod schema;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init_routes) 
    })
    .bind(format!("{}:{}", host, port))? 
    .run()
    .await
}
