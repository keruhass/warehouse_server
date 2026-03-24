use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct MaterialQuantity {
    pub material_name: String,
    pub material_quantity: Option<Decimal>,
}

