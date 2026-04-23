use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::materials::{
    get_amount_of_materials_left, get_quantity_and_sum_of_materials, post_material,
};
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
        .route("/materials/create-material", post(post_material))
}
