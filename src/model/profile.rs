use crate::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub bio: String,
    pub campaigns: Option<Vec<CampaignSummary>>,
}

impl From<User> for ProfileResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            phone: u.phone,
            bio: u.bio,
            campaigns: None
        }
    }
}

#[derive(Clone)]
pub struct ProfileUpdate {
    pub uid: i32,
    pub name: String,
    pub phone: String,
    pub bio: String,
}

pub struct ProfileBuilder {
    uid: i32,
    name: Option<String>,
    phone: Option<String>,
    bio: Option<String>,
}

impl ProfileBuilder {
    pub fn new(uid: i32) -> Self {
        Self {
            uid,
            name: None,
            phone: None,
            bio: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn build(self) -> Result<ProfileUpdate, String> {
        let name = self.name.unwrap_or_default();
        let phone = self.phone.unwrap_or_default();
        let bio = self
            .bio
            .unwrap_or_else(|| "Aku bersedia membagikan cintaku kepada dunia 💗".to_string());

        if name.trim().is_empty() {
            return Err("Name must not be empty".to_string());
        }
        if !phone.chars().all(|c| c.is_ascii_digit()) {
            return Err("Phone must contain only digits".to_string());
        }
        if bio.len() > 255 {
            return Err("Bio must be at most 255 characters".to_string());
        }

        Ok(ProfileUpdate {
            uid: self.uid,
            name,
            phone,
            bio,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignSummary {
    pub id: i32,
    pub name: String,
    pub target_amount: i64,
    pub collected_amount: i64,
    pub status: String,
}
