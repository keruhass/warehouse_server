use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::dto::document_types::DocumentType;
use crate::errors::api::ApiError;
use crate::state::AppState;

#[axum::debug_handler]
pub async fn post_document_type(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DocumentType>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!(
        "INSERT INTO document_types (document_type_name) VALUES ($1)",
        payload.document_type_name,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}
