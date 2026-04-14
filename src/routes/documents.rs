use std::sync::Arc;

use axum::routing::post;
use axum::Router;

use crate::handlers::documents::post_document;
use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/documents/create-document", post(post_document))
}
