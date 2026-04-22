use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::dto::material_groups::{CreateMaterialGroup, MaterialName};
use crate::errors::api::ApiError;
use crate::state::AppState;

pub async fn get_materials_by_group(
    State(state): State<Arc<AppState>>,
    Path(group_name): Path<String>,
) -> Result<Json<Vec<MaterialName>>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        MaterialName,
        "
        SELECT
            m.material_name
        FROM materials m
        JOIN material_groups g ON m.group_id = g.group_id
        WHERE g.group_name = $1
        ",
        group_name
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.is_empty() {
        Err((
            StatusCode::NOT_FOUND,
            format!("Материалы в группе {} не были найдены.", group_name),
        ))
    } else {
        Ok(Json(result))
    }
}

pub async fn post_material_group(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMaterialGroup>,
) -> Result<StatusCode, ApiError> {
    if payload.group_name.trim().is_empty() || payload.class_id <= 0 {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "Название группы материалов не может быть пустым, class_id должен быть положительным"
                .to_string(),
        ));
    }

    sqlx::query("INSERT INTO material_groups (group_name, class_id) VALUES ($1, $2)")
        .bind(payload.group_name)
        .bind(payload.class_id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::CREATED)
}
