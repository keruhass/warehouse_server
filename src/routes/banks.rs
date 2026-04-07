use std::sync::Arc;

use axum::routing::{get, patch};
use axum::Router;

use crate::handlers::banks::{delete_bank, get_suppliers_per_bank};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/banks/suppliers-count", get(get_suppliers_per_bank))
        .route("/banks/delete-bank", patch(delete_bank))
}
