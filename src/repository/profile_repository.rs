use crate::{
    factory::connection_factory::ConnectionFactory,
    model::profile::{ProfileResponse, ProfileUpdate},
    schema::users,
};
use diesel::prelude::*;

pub async fn find_profile_by_id(uid: i32) -> Result<ProfileResponse, String> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();

    users::table
        .filter(users::id.eq(uid))
        .first::<crate::model::user::User>(&mut conn)
        .map(ProfileResponse::from)
        .map_err(|e| e.to_string())
}

pub async fn save_profile(update: ProfileUpdate) -> Result<(), String> {
    let connection_factory = ConnectionFactory::new();
    let mut conn = connection_factory.get_connection();

    diesel::update(users::table.filter(users::id.eq(update.uid)))
        .set((
            users::name.eq(update.name),
            users::phone.eq(update.phone),
            users::bio.eq(update.bio),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
