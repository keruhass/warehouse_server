use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{
    dto::orders::{CreateOrder, OrderServiceInfo},
    errors::api::ApiError,
    state::AppState,
};

pub async fn get_order_service_info(
    State(state): State<Arc<AppState>>,
    Path(order_number): Path<String>,
) -> Result<Json<Vec<OrderServiceInfo>>, ApiError> {
    let result = sqlx::query_as!(
        OrderServiceInfo,
        "SELECT
            wm.movement_number,
            b.bank_name,
            wm.quantity * wm.unit_price AS total_amount

        FROM warehouse_movements wm

        JOIN movement_types mt 
            ON wm.movement_type_id = mt.movement_type_id

        JOIN suppliers s 
            ON wm.supplier_id = s.supplier_id

        JOIN banks b 
            ON s.bank_id = b.bank_id

        WHERE mt.movement_type_name = 'IN'
        AND wm.movement_number = $1;",
        order_number,
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Информация по данному ордеру не была найдена".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}

pub async fn post_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrder>,
) -> Result<StatusCode, ApiError> {
    if payload.movement_number.trim().is_empty()
        || payload.movement_type_id <= 0
        || payload.supplier_id <= 0
        || payload.document_id <= 0
        || payload.material_id <= 0
        || payload.unit_id <= 0
        || payload.quantity <= 0.into()
        || payload.unit_price < 0.into()
    {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Поля ордера заполнены некорректно".to_string(),
        ));
    }

    sqlx::query(
        r#"
        INSERT INTO warehouse_movements (
            movement_number,
            movement_type_id,
            supplier_id,
            document_id,
            material_id,
            unit_id,
            quantity,
            unit_price,
            movement_date
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(payload.movement_number)
    .bind(payload.movement_type_id)
    .bind(payload.supplier_id)
    .bind(payload.document_id)
    .bind(payload.material_id)
    .bind(payload.unit_id)
    .bind(payload.quantity)
    .bind(payload.unit_price)
    .bind(payload.movement_date)
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}
