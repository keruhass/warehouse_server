use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BanksSuppliersCount {
    pub bank_name: String,
    pub suppliers_count: i64,
}
