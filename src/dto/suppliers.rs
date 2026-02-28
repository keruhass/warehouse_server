use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SupplierName {
    pub supplier_name: String,
}
