use crate::module::Modules;
use crate::{
    config::Config,
    routes::{
        health::{hc, hc_mongodb},
        user::user::{get_user, list_users, post_user},
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

    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb));

    let user_router = Router::new()
        .route("/", post(post_user)) //1件登録
        .route("/", get(list_users)) //全件取得
        .route("/:id", get(get_user)); //1件取得

    let app = Router::new()
        .nest("/hc", hc_router)
        .nest("/user", user_router)
        .layer(Extension(modules))
        .layer(cors);

    let (ip_addr, port) = cfg.parse_addr_and_port().expect("Failed to parse address.");
    let addr = SocketAddr::from((ip_addr, port));
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
