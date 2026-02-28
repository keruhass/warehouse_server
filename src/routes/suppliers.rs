use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::suppliers::get_supplier_by_inn;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/suppliers/by-tax/{inn_id}", get(get_supplier_by_inn))
}
