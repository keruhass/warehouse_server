use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct MaterialName {
    pub material_name: String,
}
