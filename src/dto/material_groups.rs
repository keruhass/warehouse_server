use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct MaterialName {
    pub material_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateMaterialGroup {
    pub group_name: String,
    pub class_id: i32,
}
