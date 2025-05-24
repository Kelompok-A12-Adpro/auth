extern crate auth;

use actix_rt;
use auth::{
    model::profile::ProfileBuilder,
    service::profile_service::{get_profile, upsert_profile},
};
use dotenv::dotenv;

#[actix_rt::test]
async fn upsert_and_get_profile_success() {
    dotenv().ok();        
    let builder = ProfileBuilder::new(1)
        .name("Service Layer")
        .phone("08199887766")
        .bio("Service-test bio");

    upsert_profile(builder).await.expect("profile saved");

    let profile = get_profile(1).await.expect("profile fetched");
    assert_eq!(profile.name, "Service Layer");
    assert_eq!(profile.phone, "08199887766");
    assert_eq!(profile.bio, "Service-test bio");
}

#[actix_rt::test]
async fn upsert_profile_invalid_data_fails() {
    dotenv().ok();

    let builder = ProfileBuilder::new(1)
        .name("")           
        .phone("08BAD")
        .bio("x");

    let err = upsert_profile(builder).await.expect_err("should fail");
    assert_eq!(err, "Name must not be empty");
}
