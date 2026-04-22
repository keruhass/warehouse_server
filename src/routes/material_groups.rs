use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::material_groups::{get_materials_by_group, post_material_group};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/material_groups/assortment-by-group-name/{group_name}",
            get(get_materials_by_group),
        )
        .route(
            "/material_groups/create-material-group",
            post(post_material_group),
        )
}
