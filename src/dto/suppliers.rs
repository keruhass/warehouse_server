use rust_decimal::Decimal;
use serde::Serialize;

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
