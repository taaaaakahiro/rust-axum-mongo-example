use crate::{
    config::Config,
    routes::{
        health::{hc, html},
        user::user::{create_user, get_user},
    },
};
use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(cfg: &Config) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(html))
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/hc", get(hc))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
