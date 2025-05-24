use auth::{
    repository::profile_repository::{find_profile_by_id, save_profile},
    model::profile::ProfileBuilder,
};
use dotenv::dotenv;

#[actix_rt::test]
async fn test_save_and_get_profile() {
    dotenv().ok();
    let dto = ProfileBuilder::new(1)
        .name("Test User")
        .phone("0812345678")
        .bio("Test Bio")
        .build()
        .unwrap();

    save_profile(dto.clone()).await.expect("save");

    let fetched = find_profile_by_id(1).await.expect("find");
    assert_eq!(fetched.name, "Test User");
    assert_eq!(fetched.bio, "Test Bio");
}
