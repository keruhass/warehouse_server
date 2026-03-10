use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{dto::suppliers::SupplierName, state::AppState};

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
