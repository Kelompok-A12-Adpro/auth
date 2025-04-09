use crate::models::user::{User, NewUser};
use diesel::prelude::*;
use crate::schema::users;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub async fn find_user_by_email(email: &str) -> Option<User> {
    let conn = establish_connection().await;
    users::table
        .filter(users::email.eq(email))
        .first::<User>(&conn)
        .optional()
        .unwrap()
}

pub async fn create_user(new_user: User) -> Result<(), String> {
    let conn = establish_connection().await;

    let new_user = NewUser {
        email: &new_user.email,
        password: &new_user.password,
        name: &new_user.name,
        phone: &new_user.phone,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");
    pool.get().expect("Failed to get DB connection from pool")
}
