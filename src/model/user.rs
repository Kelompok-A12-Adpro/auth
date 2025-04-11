use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub phone: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
