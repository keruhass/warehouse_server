use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::dto::material_groups::MaterialName;
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
