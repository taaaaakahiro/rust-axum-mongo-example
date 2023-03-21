use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html};

use crate::module::{Modules, ModulesExt};
use axum::Extension;
use std::sync::Arc;
use tracing::error;

pub async fn hc() -> impl IntoResponse {
    print!("hello world!");
    tracing::debug!("Access health check endpoint.");
    StatusCode::NO_CONTENT
}

pub async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn hc_mongodb(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .health_check_use_case()
        .diagnose_mongo_db_conn()
        .await
        .map(|_| {
            tracing::debug!("Access mongodb health check endpoint.");
            StatusCode::NO_CONTENT
        })
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::SERVICE_UNAVAILABLE
        })
}
