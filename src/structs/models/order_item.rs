use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub amount: isize,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Uuid,
    pub variant_choice_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct OrderItemNew {
    pub amount: isize,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Uuid,
    pub variant_choice_id: Uuid,
}

#[derive(Deserialize)]
pub struct OrderItemUpdate {
    pub amount: Option<isize>,
    pub order_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub variant_id: Option<Uuid>,
    pub variant_choice_id: Option<Uuid>,
}
