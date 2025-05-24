use auth::model::profile::ProfileBuilder;

#[test]
fn test_builder_valid() {
    let dto = ProfileBuilder::new(1)
        .name("Alice")
        .phone("08123456789")
        .bio("Hello world")
        .build()
        .expect("should build");

    assert_eq!(dto.uid, 1);
    assert_eq!(dto.name, "Alice");
    assert_eq!(dto.phone, "08123456789");
    assert_eq!(dto.bio, "Hello world");
}

#[test]
fn test_builder_invalid_phone() {
    let res = ProfileBuilder::new(1)
        .name("Bob")
        .phone("invalid-phone")
        .build();
    assert!(res.is_err());
}
