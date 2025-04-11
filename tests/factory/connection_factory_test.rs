use crate::factory::connection_factory::{ConnectionFactory, get_connection_factory};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_env() {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    }

    #[test]
    fn test_create_connection_factory() {
        ensure_env();
        let factory = ConnectionFactory::new();

        let conn = factory.get_connection();

        let pool = factory.get_pool();
        assert_eq!(pool.state().max_size, 15);
    }
    
    #[test]
    fn test_get_connection_factory() {
        ensure_env();
        let factory = get_connection_factory();

        let _conn = factory.get_connection();
        
        let factory2 = get_connection_factory();

        let pool1 = factory.get_pool();
        let pool2 = factory2.get_pool();
        
        assert_eq!(pool1.state().max_size, pool2.state().max_size);
    }

    #[test]
    fn test_connection_pool_config() {
        ensure_env();
        let factory = ConnectionFactory::new();
        let pool = factory.get_pool();
        
        let state = pool.state();
        assert_eq!(state.max_size, 15);
    }
    
    #[test]
    fn test_multiple_connections() {
        ensure_env();
        let factory = ConnectionFactory::new();

        let conn1 = factory.get_connection();
        let conn2 = factory.get_connection();
        let conn3 = factory.get_connection();
        
        drop(conn1);
        drop(conn2);
        drop(conn3);
        
        let _conn4 = factory.get_connection();
    }
}~