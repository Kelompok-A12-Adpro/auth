use crate::factory::connection_factory::ConnectionFactory;
use crate::model::user::{User, NewUser};
use diesel::prelude::*;
use crate::schema::users;

pub async fn find_user_by_email(email: &str) -> Option<User> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();
    
    users::table
        .filter(users::email.eq(email))
        .first::<User>(&mut conn)
        .optional()
        .unwrap()
}

pub async fn create_user(new_user: User) -> Result<(), String> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();
    
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