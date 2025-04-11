use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct ConnectionFactory {
    pool: DbPool,
}

impl ConnectionFactory {
    pub fn new() -> Self {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");
        
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create database connection pool");
        
        ConnectionFactory { pool }
    }
    
    pub fn get_connection(&self) -> DbConnection {
        self.pool.get().expect("Failed to get connection from pool")
    }
    
    pub fn get_pool(&self) -> DbPool {
        self.pool.clone()
    }
}

lazy_static::lazy_static! {
    static ref CONNECTION_FACTORY: ConnectionFactory = ConnectionFactory::new();
}

pub fn get_connection_factory() -> &'static ConnectionFactory {
    &*CONNECTION_FACTORY
}