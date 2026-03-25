use serde::Serialize;
use sqlx::types::Decimal;

#[derive(Debug, Serialize, Clone)]
pub struct AmountOfMoneyPerBank {
    pub bank_name: String,
    pub total_money: Option<Decimal>,
}
#[derive(Debug, Serialize, Clone)]
pub struct MonthTurnover {
    pub month: Option<i32>,
    pub total_turnover: Option<Decimal>,
}
