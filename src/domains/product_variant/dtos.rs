use serde::Deserialize;
use sqlx::types::Uuid;

#[derive(Deserialize)]
pub struct ProductVariantNew {
    pub name: String,
    pub description: String,
    pub product_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProductVariantUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}
