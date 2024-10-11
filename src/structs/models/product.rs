use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ProductNew<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: f64,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProductUpdate<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub price: Option<f64>,
}
