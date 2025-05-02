use auth::factory;
use auth::routes;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use factory::connection_factory::ConnectionFactory;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();

    let connection_factory = ConnectionFactory::new();
    let pool = connection_factory.get_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::routes::init_routes) 
    })
    .bind(format!("{}:{}", host, port))? 
    .run()
    .await
}