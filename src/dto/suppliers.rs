use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Supplier {
    pub supplier_id: i32,
    pub supplier_name: String,
    pub inn: String,
    pub legal_post_index: Option<String>,
    pub legal_city: Option<String>,
    pub legal_street: Option<String>,
    pub legal_house: Option<String>,
    pub bank_id: i32,
    pub bank_account: String,
}
#[derive(Debug, Serialize, Clone)]
pub struct SupplierName {
    pub supplier_name: String,
}
#[derive(Debug, Serialize, Clone)]
pub struct SupplierShare {
    pub supplier_name: String,
    pub supplier_quantity: Option<Decimal>,
    pub supplier_share: Option<Decimal>,
}
