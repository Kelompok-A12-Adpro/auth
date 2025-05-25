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

pub async fn update_profile(
    uid: i32,
    new_name: &str,
    new_phone: &str,
    new_bio: &str,
) -> Result<(), String> {
    let factory = ConnectionFactory::new();
    let mut conn = factory.get_connection();

    diesel::update(users::table.filter(users::id.eq(uid)))
        .set((
            users::name.eq(new_name),
            users::phone.eq(new_phone),
            users::bio.eq(new_bio),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn find_user_by_id(uid: i32) -> Option<crate::model::user::User> {
    let factory = ConnectionFactory::new();
    let mut conn = factory.get_connection();

    users::table
        .filter(users::id.eq(uid))
        .first::<crate::model::user::User>(&mut conn)
        .optional()
        .unwrap()
}