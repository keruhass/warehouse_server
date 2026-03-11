use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::banks::get_suppliers_per_bank;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/banks/suppliers-count", get(get_suppliers_per_bank))
}
