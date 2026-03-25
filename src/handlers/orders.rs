use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::{dto::orders::OrderServiceInfo, errors::api::ApiError, state::AppState};

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
