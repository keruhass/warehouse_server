use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DocumentType {
    pub document_type_name: String,
}
