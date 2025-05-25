use crate::{
    model::profile::{ProfileBuilder, ProfileResponse},
    repository::profile_repository::{find_profile_by_id, save_profile},
};

pub async fn get_profile(uid: i32) -> Result<ProfileResponse, String> {
    find_profile_by_id(uid).await
}

pub async fn upsert_profile(builder: ProfileBuilder) -> Result<(), String> {
    let dto = builder.build()?;
    save_profile(dto).await
}
