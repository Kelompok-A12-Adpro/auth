use diesel::prelude::*;
use crate::schema::users;
use crate::model::user::{NewUser, User, UserDataResponse};
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

pub async fn get_recent_users_and_total() -> Result<UserDataResponse, String> {
    let factory = ConnectionFactory::new();
    let mut conn = factory.get_connection();

    // Get total count of users
    let total_users = users::table
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Get 5 most recent users (highest IDs)
    let recent_users = users::table
        .order(users::id.desc())
        .limit(5)
        .load::<User>(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(UserDataResponse {
        users: recent_users,
        total: total_users,
    })
}

pub async fn get_all_users() -> Result<UserDataResponse, String> {
    let factory = ConnectionFactory::new();
    let mut conn = factory.get_connection();

    let users = users::table
        .order(users::id.desc())
        .load::<User>(&mut conn)
        .map_err(|e| e.to_string())?;

    let total_users = users.len() as i64;

    Ok(UserDataResponse {
        users,
        total: total_users,
    })
}

pub async fn delete_user(uid: i32) -> Result<(), String> {
    let factory = ConnectionFactory::new();
    let mut conn = factory.get_connection();

    diesel::delete(users::table.filter(users::id.eq(uid)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}