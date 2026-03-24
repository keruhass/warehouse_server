use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::materials::{get_amount_of_materials_left, get_quantity_and_sum_of_materials};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/materials/amount_of_materials_left/{range}",
            get(get_amount_of_materials_left),
        )
        .route(
            "/materials/total_amount_and_sum",
            get(get_quantity_and_sum_of_materials),
        )
}
