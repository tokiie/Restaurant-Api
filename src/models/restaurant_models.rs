use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rust_decimal::Decimal;


#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Items {
    pub id: Uuid,
    pub tables_id: Uuid,
    pub menu_id: Uuid,
    pub quantity: i32,
    pub delivered_quantity: i32,
    pub delivered_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialItem {
    pub id: Uuid,
    pub tables_id: Uuid,
    pub menu_id: Uuid,
    pub quantity: i32,
    pub delivered_quantity: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialItemReturn {
    pub id: Uuid,
    pub tables_id: Uuid,
    pub menu_id: Uuid,
    pub quantity: i32,
    pub delivered_quantity: i32,
    pub created_at: NaiveDateTime,
    pub prep_time: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Menu {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub prep_time: i32,
}