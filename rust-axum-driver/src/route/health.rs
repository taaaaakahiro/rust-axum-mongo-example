use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn hc() -> impl IntoResponse {
    tracing::debug!("Access health check endpoint.");
    StatusCode::NO_CONTENT
}
