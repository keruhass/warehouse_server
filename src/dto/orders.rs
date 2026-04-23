use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

#[derive(Serialize, Clone)]
pub struct OrderServiceInfo {
    pub movement_number: String,
    pub bank_name: String,
    pub total_amount: Option<Decimal>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrder {
    pub movement_number: String,
    pub movement_type_id: i32,
    pub supplier_id: i32,
    pub document_id: i32,
    pub material_id: i32,
    pub unit_id: i32,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub movement_date: NaiveDate,
}
