use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

use auth::factory;
use auth::routes;
use factory::connection_factory::ConnectionFactory;

use auth::metrics::setup_metrics;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let prometheus = setup_metrics();

    let connection_factory = ConnectionFactory::new();
    let pool = connection_factory.get_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000") // Ganti dengan URL frontend saat deploy
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(prometheus.clone())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::routes::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
