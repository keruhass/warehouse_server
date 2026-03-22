use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use crate::errors::api::ApiError;
use crate::{dto::service::AmountOfMoneyPerBank, state::AppState};

#[axum::debug_handler]
pub async fn get_amount_of_money_per_bank(
    State(state): State<Arc<AppState>>,
    Path(range): Path<String>,
) -> Result<Json<Vec<AmountOfMoneyPerBank>>, ApiError> {
    let (start_str, end_str) = range.split_once('-').ok_or(ApiError(
        StatusCode::BAD_REQUEST,
        "Cannot split the range of dates".to_string(),
    ))?;

    let start_date = NaiveDate::parse_from_str(start_str, "%Y.%m.%d")?;
    let end_date = NaiveDate::parse_from_str(end_str, "%Y.%m.%d")?;
    println!("Date 1: {}", start_date);
    println!("Date 2: {}", end_date);

    let result = sqlx::query_as!(
        AmountOfMoneyPerBank,
        "SELECT
            b.bank_name,
        COALESCE(SUM(wm.quantity * wm.unit_price), 0) AS total_money
        FROM warehouse_movements wm
        JOIN movement_types mt 
        ON wm.movement_type_id = mt.movement_type_id
        JOIN suppliers s 
        ON wm.supplier_id = s.supplier_id
        JOIN banks b 
        ON s.bank_id = b.bank_id
        WHERE mt.movement_type_name = 'IN'
        AND wm.movement_date BETWEEN $1 AND $2
        GROUP BY b.bank_name
        ORDER BY total_money DESC;",
        start_date,
        end_date,
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Банки не были найдены".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}
