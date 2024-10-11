use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Deserialize, Serialize)]
pub enum OrderStatus {
    PendingPayment,
    Paid,
    ReadyToShip,
    Cancelled,
    Delivered,
}

#[derive(FromRow, Deserialize, Serialize)]
pub struct Order {
    pub id: Uuid,
    pub note: Option<String>,
    pub status: OrderStatus,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewOrder {
    pub note: Option<String>,
    pub users_id: Uuid,
    pub status: OrderStatus,
}

pub struct OrderUpdate {
    pub note: Option<String>,
    pub status: Option<OrderStatus>,
}
