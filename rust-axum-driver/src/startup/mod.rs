use crate::module::Modules;
use crate::{
    config::Config,
    routes::{
        health::{hc, hc_mongodb, html},
        user::user::{get_user, post_user},
    },
};
use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(cfg: &Config, modules: Arc<Modules>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb))
        .route("/html", get(html))
        .route("/user", post(post_user))
        .route("/user/:id", get(get_user))
        .route("/hc", get(hc))
        .layer(Extension(modules))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::info!("Server Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
