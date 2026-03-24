use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::suppliers::{
    get_supplier_by_inn, get_suppliers, get_suppliers_by_bank, get_suppliers_share,
};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/suppliers/by-inn/{inn_id}", get(get_supplier_by_inn))
        .route("/suppliers", get(get_suppliers))
        .route(
            "/suppliers/by-bank-name/{bank_name}",
            get(get_suppliers_by_bank),
        )
        .route("/suppliers/share/{group_name}", get(get_suppliers_share))
}
