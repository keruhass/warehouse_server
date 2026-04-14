use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{dto::documents::Document, errors::api::ApiError, state::AppState};

pub async fn post_document(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Document>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!(
        "INSERT INTO documents (document_type_id, document_number, document_date) VALUES ($1, $2, $3)",
        payload.document_type_id,
        payload.document_number,
        payload.document_date,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}
