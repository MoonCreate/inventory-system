use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Deserialize, Serialize, FromRow)]
pub struct ProductVariantItem {
    pub id: Uuid,
    pub name: String,
    pub price_mutation: f64,
    pub variant_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ProductVariantItemNew<'a> {
    pub name: &'a str,
    pub price_mutation: f64,
    pub variant_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProductVariantItemUpdate<'a> {
    pub name: Option<&'a str>,
    pub price_mutation: Option<f64>,
    pub variant_id: Option<Uuid>,
}
