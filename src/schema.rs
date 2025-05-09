// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        phone -> Varchar,
        is_admin -> Bool,
    }
}
