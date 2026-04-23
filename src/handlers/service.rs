use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;

use crate::{dto::service::MonthTurnover, errors::api::ApiError};
use crate::{
    dto::service::{AmountOfMoneyPerBank, CreateUnit},
    state::AppState,
};

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
pub async fn get_year_turnover(
    State(state): State<Arc<AppState>>,
    Path(request): Path<String>,
) -> Result<Json<Vec<MonthTurnover>>, ApiError> {
    let year = Decimal::from_str(&request).unwrap_or(Decimal::ZERO);
    let result = sqlx::query_as!(
        MonthTurnover,
        "SELECT
            EXTRACT(MONTH FROM wm.movement_date)::INT AS month,

            SUM(
                CASE
                    WHEN mt.movement_type_name = 'IN' THEN wm.quantity
                    ELSE wm.quantity
                END
            ) AS total_turnover

        FROM warehouse_movements wm

        JOIN movement_types mt 
            ON wm.movement_type_id = mt.movement_type_id

        WHERE EXTRACT(YEAR FROM wm.movement_date) = $1

        GROUP BY month

        ORDER BY month;",
        year,
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Информация о загруженности календарного года не была найдена".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}

pub async fn post_unit(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUnit>,
) -> Result<StatusCode, ApiError> {
    sqlx::query("INSERT INTO units (unit_name) VALUES ($1)")
        .bind(payload.unit_name)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::CREATED)
}
