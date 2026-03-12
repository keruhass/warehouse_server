use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::material_groups::get_materials_by_group;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/material_groups/assortment-by-group-name/{group_name}",
        get(get_materials_by_group),
    )
}
