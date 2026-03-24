use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::materials::get_amount_of_materials_left;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/materials/amount_of_materials_left/{range}",
        get(get_amount_of_materials_left),
    )
}
