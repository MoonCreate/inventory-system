use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    types::Uuid,
};

#[derive(Serialize, Deserialize, Clone, Copy, Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Administrator,
    Buyer,
    Seller,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: UserRole,
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
    pub role: Option<UserRole>,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bcrypt::BcryptResult<bool> {
        Ok(bcrypt::verify(password, &self.password)?)
    }
}
