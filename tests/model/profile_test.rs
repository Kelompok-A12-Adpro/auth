extern crate auth;

use auth::model::profile::ProfileBuilder;

#[test]
fn build_success_with_all_fields() {
    let dto = ProfileBuilder::new(42)
        .name("Jane Doe")
        .phone("081234567890")
        .bio("Rustacean at heart 🦀")
        .build()
        .expect("should build");

    assert_eq!(dto.uid, 42);
    assert_eq!(dto.name, "Jane Doe");
    assert_eq!(dto.phone, "081234567890");
    assert_eq!(dto.bio, "Rustacean at heart 🦀");
}

#[test]
fn build_success_with_default_bio() {
    let dto = ProfileBuilder::new(1)
        .name("John")
        .phone("0800000000")
        .build()
        .expect("should build with default bio");

    assert_eq!(
        dto.bio,
        "Aku bersedia membagikan cintaku kepada dunia 💗"
    );
}

#[test]
fn build_fails_empty_name() {
    let res = ProfileBuilder::new(1)
        .name("")         
        .phone("0812")
        .bio("hi")
        .build();

    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Name must not be empty");
}

#[test]
fn build_fails_non_digit_phone() {
    let res = ProfileBuilder::new(1)
        .name("A")
        .phone("08ABCD")
        .build();

    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Phone must contain only digits");
}

#[test]
fn build_fails_bio_too_long() {
    let long_bio = "x".repeat(256);
    let res = ProfileBuilder::new(1)
        .name("A")
        .phone("0812")
        .bio(long_bio)
        .build();

    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Bio must be at most 255 characters");
}
