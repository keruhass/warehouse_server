use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub mod suppliers;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(suppliers::router())
}
