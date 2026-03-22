use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::handlers::service::get_amount_of_money_per_bank;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/service/get-amount-of-money/per-bank/per-period/{range}",
        get(get_amount_of_money_per_bank),
    )
}
