use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::service::{get_amount_of_money_per_bank, get_year_turnover, post_unit};
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/service/get-amount-of-money/per-bank/per-period/{range}",
            get(get_amount_of_money_per_bank),
        )
        .route("/service/year-turnover/{request}", get(get_year_turnover))
        .route("/service/create-unit", post(post_unit))
}
