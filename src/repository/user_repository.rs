use diesel::prelude::*;
use crate::schema::users;
use crate::model::user::{User, NewUser};
use crate::factory::connection_factory::ConnectionFactory;

pub async fn find_user_by_email(email: &str) -> Option<User> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();

    users::table
        .filter(users::email.eq(email))
        .first::<User>(&mut conn)
        .optional()
        .unwrap()
}

pub async fn create_user(new_user: NewUser<'_>) -> Result<(), String> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
