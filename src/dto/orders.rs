use serde::Serialize;
use sqlx::types::Decimal;

#[derive(Serialize, Clone)]
pub struct OrderServiceInfo {
    pub movement_number: String,
    pub bank_name: String,
    pub total_amount: Option<Decimal>,
}
