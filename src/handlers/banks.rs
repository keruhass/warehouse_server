use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::errors::api::ApiError;
use crate::{dto::banks::BanksSuppliersCount, state::AppState};

pub async fn get_suppliers_per_bank(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BanksSuppliersCount>>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        BanksSuppliersCount,
        r#"
        SELECT
           b.bank_name AS bank_name,
           COUNT(s.supplier_id) AS "suppliers_count!: i64"
        FROM banks b
        LEFT JOIN suppliers s ON s.bank_id = b.bank_id
        GROUP BY b.bank_name;
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.is_empty() {
        Err((StatusCode::NOT_FOUND, "Банки не найдены.".to_string()))
    } else {
        Ok(Json(result))
    }
}

pub async fn delete_bank(State(state): State<Arc<AppState>>) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM banks
        WHERE bank_id = (
            SELECT b.bank_id
            FROM banks b
            JOIN suppliers s ON s.bank_id = b.bank_id
            GROUP BY b.bank_id
            ORDER BY COUNT(s.supplier_id) DESC
            LIMIT 1
        );
        "#
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
