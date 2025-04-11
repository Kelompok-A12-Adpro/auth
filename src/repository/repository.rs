use crate::models::{User, NewUser};
use diesel::prelude::*;
use crate::schema::users;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub async fn find_user_by_email(email: &str) -> Option<User> {
    let mut conn = establish_connection();
    users::table
        .filter(users::email.eq(email))
        .first::<User>(&mut conn)
        .optional()
        .unwrap()
}

pub async fn create_user(new_user: User) -> Result<(), String> {
    let mut conn = establish_connection();

    let new_user = NewUser {
        email: &new_user.email,
        password: &new_user.password,
        name: &new_user.name,
        phone: &new_user.phone,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool.get().expect("Failed to get DB connection from pool")
}
