use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub mod banks;
pub mod material_groups;
pub mod materials;
pub mod service;
pub mod suppliers;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .merge(suppliers::router())
        .merge(banks::router())
        .merge(material_groups::router())
        .merge(service::router())
        .merge(materials::router())
}
