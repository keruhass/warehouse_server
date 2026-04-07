use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    dto::suppliers::{Supplier, SupplierName, SupplierShare},
    errors::api::ApiError,
    state::AppState,
};

pub async fn get_supplier_by_id(
    State(state): State<Arc<AppState>>,
    Path(supplier_id): Path<i32>,
) -> Result<Json<Supplier>, ApiError> {
    let result = sqlx::query_as!(
        Supplier,
        "SELECT * FROM suppliers WHERE supplier_id = $1",
        supplier_id
    )
    .fetch_optional(&state.db)
    .await?;

    match result {
        Some(supplier) => Ok(Json(supplier)),
        None => Err(ApiError(
            StatusCode::NOT_FOUND,
            "Поставщик с указанным id не был найден".to_string(),
        )),
    }
}
pub async fn get_suppliers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SupplierName>>, (StatusCode, String)> {
    let result = sqlx::query_as!(SupplierName, "SELECT supplier_name FROM suppliers",)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.is_empty() {
        Err((StatusCode::NOT_FOUND, "Поставщики не найдены.".to_string()))
    } else {
        Ok(Json(result))
    }
}
// 1. GET /api/suppliers/by-inn/inn
pub async fn get_supplier_by_inn(
    State(state): State<Arc<AppState>>,
    Path(inn_id): Path<String>,
) -> Result<Json<SupplierName>, (StatusCode, String)> {
    if let Some(name) = state.supplier_cache.get(&inn_id) {
        return Ok(Json(SupplierName {
            supplier_name: name.clone(),
        }));
    }

    let result = sqlx::query_as!(
        SupplierName,
        "SELECT supplier_name FROM suppliers WHERE inn = $1",
        inn_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match result {
        Some(supplier) => {
            state
                .supplier_cache
                .insert(inn_id, supplier.supplier_name.clone());
            Ok(Json(supplier))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Поставщик с ИНН {} не был найден.", inn_id),
        )),
    }
}

pub async fn get_suppliers_by_bank(
    State(state): State<Arc<AppState>>,
    Path(bank_id): Path<String>,
) -> Result<Json<Vec<SupplierName>>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        SupplierName,
        "
        SELECT
            s.supplier_name
        FROM suppliers s
        JOIN banks b ON s.bank_id = b.bank_id 
        WHERE b.bank_name = $1
        ",
        bank_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.is_empty() {
        Err((
            StatusCode::NOT_FOUND,
            format!("Поставщики с банком {} не были найдены.", bank_id),
        ))
    } else {
        Ok(Json(result))
    }
}
pub async fn get_suppliers_share(
    State(state): State<Arc<AppState>>,
    Path(group_name): Path<String>,
) -> Result<Json<Vec<SupplierShare>>, ApiError> {
    let result = sqlx::query_as!(
        SupplierShare,
        "SELECT
            s.supplier_name,
            SUM(wm.quantity) AS supplier_quantity,

            SUM(wm.quantity) * 1.0 
            / SUM(SUM(wm.quantity)) OVER () AS supplier_share

        FROM warehouse_movements wm

        JOIN movement_types mt 
            ON wm.movement_type_id = mt.movement_type_id

        JOIN suppliers s 
            ON wm.supplier_id = s.supplier_id

        JOIN materials m 
            ON wm.material_id = m.material_id

        JOIN material_groups mg 
            ON m.group_id = mg.group_id

        WHERE mt.movement_type_name = 'IN'
        AND mg.group_name = $1   -- параметр: название группы

        GROUP BY s.supplier_name

        ORDER BY supplier_share DESC;",
        group_name,
    )
    .fetch_all(&state.db)
    .await?;

    if result.is_empty() {
        Err(ApiError(
            StatusCode::NOT_FOUND,
            "Поставщики не были найдены".to_string(),
        ))
    } else {
        Ok(Json(result))
    }
}

pub async fn update_inn(State(state): State<Arc<AppState>>) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!(
        r#"
        UPDATE suppliers
        SET inn =
            CONCAT(
                ((SUBSTRING(inn FROM 1 FOR 1)::INT + 1) % 10),
                ((SUBSTRING(inn FROM 2 FOR 1)::INT + 1) % 10),
                ((SUBSTRING(inn FROM 3 FOR 1)::INT + 1) % 10),
                SUBSTRING(inn FROM 4)
            ) 
        "#
    )
    .execute(&state.db)
    .await?;

    println!("Updated rows: {}", result.rows_affected());

    Ok(StatusCode::NO_CONTENT)
}
