use crate::{
    model::profile::{ProfileBuilder, ProfileResponse},
    repository::profile_repository::{find_profile_by_id, save_profile},
};

pub async fn get_profile(uid: i32) -> Result<ProfileResponse, String> {
    let profile = find_profile_by_id(uid).await?;
    let campaigns = fetch_user_campaigns(uid).await;

    Ok(ProfileResponse {
        id: profile.id,
        name: profile.name,
        email: profile.email,
        phone: profile.phone,
        bio: profile.bio,
        campaigns: Some(campaigns),
    })
}


pub async fn upsert_profile(builder: ProfileBuilder) -> Result<(), String> {
    let dto = builder.build()?;
    save_profile(dto).await
}

use crate::model::profile::{CampaignSummary};
use reqwest;
use serde::Deserialize;

pub async fn fetch_user_campaigns(user_id: i32) -> Vec<CampaignSummary> {
    let url = format!("http://localhost:8000/campaigns/user/{}", user_id); // adjust port as needed

    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<Vec<CampaignSummary>>().await {
            Ok(campaigns) => campaigns,
            Err(_) => vec![],
        },
        Err(_) => vec![],
    }
}
