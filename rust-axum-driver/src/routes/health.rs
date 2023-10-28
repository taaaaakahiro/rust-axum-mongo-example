use crate::module::{Modules, ModulesExt};
use axum::response::IntoResponse;
use axum::Extension;
use axum::{http::StatusCode, response::Html};
use std::sync::Arc;
use tracing::error;

pub async fn hc() -> impl IntoResponse {
    tracing::info!("Success: Access health check endpoint.");
    StatusCode::OK
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
            tracing::info!("Success: Access mongodb health check endpoint.");
            StatusCode::OK
        })
        .map_err(|err| {
            tracing::info!("Fail: Access mongodb health check endpoint.");
            error!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
