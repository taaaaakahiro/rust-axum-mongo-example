use crate::config::Config;
use crate::routes::{health::html, user::user};

use axum::http::Method;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(cfg: &Config) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS, Method::HEAD])
        .allow_origin(Any);

    let app = Router::new()
        .layer(cors)
        .route("/", get(html))
        .route("/user", get(user::get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
