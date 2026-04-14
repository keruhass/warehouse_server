use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Document {
    pub document_type_id: i32,
    pub document_number: String,
    pub document_date: NaiveDate,
}
