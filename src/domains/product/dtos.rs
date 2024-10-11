use serde::Deserialize;
use sqlx::types::Uuid;

#[derive(Deserialize)]
pub struct UpdateProductById {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub user_id: Uuid,
}
