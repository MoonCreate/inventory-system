use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct UserNew<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct UserUpdate<'a> {
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}
