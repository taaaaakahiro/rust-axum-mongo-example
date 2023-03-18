use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html};

pub async fn hc() -> impl IntoResponse {
    print!("hello world!");
    tracing::debug!("Access health check endpoint.");
    StatusCode::NO_CONTENT
}

pub async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
