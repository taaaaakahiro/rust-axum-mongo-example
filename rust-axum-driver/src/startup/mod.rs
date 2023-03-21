use crate::config::Config;
use crate::module::Modules;
use crate::routes::{health::hc_mongodb, health::html, user::user};
use axum::http::Method;
use axum::{routing::get, Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(cfg: &Config, modules: Arc<Modules>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS, Method::HEAD])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(html))
        .route("/user", get(user::get_user))
        .route("/mongo_hc", get(hc_mongodb))
        .layer(cors)
        .layer(Extension(modules));

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
