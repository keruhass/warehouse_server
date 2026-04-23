use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct MaterialQuantity {
    pub material_name: String,
    pub material_quantity: Option<Decimal>,
}
#[derive(Debug, Serialize, Clone)]
pub struct MaterialQuantityAndTotalSum {
    pub material_name: String,
    pub material_quantity: Option<Decimal>,
    pub total_sum: Option<Decimal>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct CreateMaterial {
    pub material_name: String,
    pub group_id: i32,
}
