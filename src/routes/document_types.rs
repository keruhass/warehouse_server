use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::document_types::post_document_type, state::AppState};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route(
        "/document-types/create-document-type",
        post(post_document_type),
    )
}
