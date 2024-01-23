use crate::module::Modules;
use crate::{
    config::Config,
    routes::{
        health::{hc, hc_mongodb},
        user::user::{find_one, post_user},
    },
};
use axum::{
    http::Method,
    routing::{get, post},
    serve, Extension, Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(cfg: &Config, modules: Arc<Modules>) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::OPTIONS]);

    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb));

    let user_router = Router::new()
        .route("/", post(post_user)) //1件登録
        .route("/:id", get(find_one)); //1件取得

    let app = Router::new()
        .nest("/hc", hc_router)
        .nest("/user", user_router)
        .layer(Extension(modules))
        .layer(cors);

    let (ip_addr, port) = cfg.parse_addr_and_port().expect("failed to parse address");
    let addr = SocketAddr::from((ip_addr, port));
    tracing::info!("Server Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(format!("{}", addr))
        .await
        .unwrap();

    serve(listener, app).await.expect("failed to serve api");
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
