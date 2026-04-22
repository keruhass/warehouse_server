use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct BanksSuppliersCount {
    pub bank_name: String,
    pub suppliers_count: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateBank {
    pub bank_name: String,
    pub post_index: String,
    pub city: String,
    pub street: String,
    pub house: String,
}
