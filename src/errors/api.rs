use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::ParseError;

#[derive(Debug, Clone)]
pub struct ApiError(pub StatusCode, pub String);

impl From<ParseError> for ApiError {
    fn from(e: ParseError) -> Self {
        ApiError(
            StatusCode::BAD_REQUEST,
            format!("Invalid date format: {}", e),
        )
    }
}
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let ApiError(status, message) = self;

        let body = Json(serde_json::json!({ "error": message }));

        (status, body).into_response()
    }
}
