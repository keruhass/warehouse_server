use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use crate::errors::api::ApiError;
use crate::{
    dto::materials::{CreateMaterial, MaterialQuantity, MaterialQuantityAndTotalSum},
    state::AppState,
};

pub async fn get_amount_of_materials_left(
    State(state): State<Arc<AppState>>,
    Path(range): Path<String>,
) -> Result<Json<Vec<MaterialQuantity>>, ApiError> {
    let (start_str, end_str) = range.split_once('-').ok_or(ApiError(
        StatusCode::BAD_REQUEST,
        "Cannot split the range of dates".to_string(),
    ))?;

    let start_date = NaiveDate::parse_from_str(start_str, "%Y.%m.%d")?;
    let end_date = NaiveDate::parse_from_str(end_str, "%Y.%m.%d")?;

    let result = sqlx::query_as!(
        MaterialQuantity,
        "SELECT
            m.material_name,
            SUM(wm.quantity) AS material_quantity
        FROM warehouse_movements wm
        JOIN movement_types mt 
            ON wm.movement_type_id = mt.movement_type_id
        JOIN materials m 
            ON wm.material_id = m.material_id
        WHERE mt.movement_type_name = 'OUT'
        AND wm.movement_date BETWEEN $1 AND $2
        GROUP BY m.material_name
        ORDER BY material_quantity DESC;",
        start_date,
        end_date,
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Материалы не были найдены".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}
pub async fn get_quantity_and_sum_of_materials(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MaterialQuantityAndTotalSum>>, ApiError> {
    let result = sqlx::query_as!(
        MaterialQuantityAndTotalSum,
        "SELECT
            m.material_name,
            COALESCE(
                SUM(
                    CASE
                        WHEN mt.movement_type_name = 'IN' THEN wm.quantity
                        ELSE -wm.quantity
                    END
                ), 
            0) AS material_quantity,
    
            COALESCE(
                SUM(
                    CASE
                        WHEN mt.movement_type_name = 'IN' THEN wm.quantity * wm.unit_price
                        ELSE -wm.quantity * wm.unit_price
                    END
                ), 
            0) AS total_sum

            FROM warehouse_movements wm

            JOIN movement_types mt 
                ON wm.movement_type_id = mt.movement_type_id

            JOIN materials m 
                ON wm.material_id = m.material_id

            GROUP BY m.material_name

            ORDER BY m.material_name;",
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Материалы не были найдены".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}

pub async fn post_material(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMaterial>,
) -> Result<StatusCode, ApiError> {
    if payload.material_name.trim().is_empty() || payload.group_id <= 0 {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Имя материала должно быть непустым, а group_id должен быть положительным".to_string(),
        ));
    }

    sqlx::query("INSERT INTO materials (material_name, group_id) VALUES ($1, $2)")
        .bind(payload.material_name)
        .bind(payload.group_id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::CREATED)
}
