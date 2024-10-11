use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ProductVariantNew<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub product_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProductVariantUpdate<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}
