use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::orders::{get_order_service_info, post_order};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/orders/order-service-info/{order_number}",
            get(get_order_service_info),
        )
        .route("/orders/create-order", post(post_order))
}
