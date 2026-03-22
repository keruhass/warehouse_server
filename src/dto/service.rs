use serde::Serialize;
use sqlx::types::Decimal;

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct AmountOfMoneyPerBank {
    pub bank_name: String,
    pub total_money: Option<Decimal>,
}
