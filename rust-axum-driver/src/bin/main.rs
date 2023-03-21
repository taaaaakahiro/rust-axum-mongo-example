use std::sync::Arc;

use rust_axum_driver::{config::Config, module::Modules, startup::startup};

#[tokio::main] //main関数を非同期関数にする
async fn main() {
    // init
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();
    let cfg = Config::new();
    let modules = Modules::new().await;
    let _ = startup(&cfg, Arc::new(modules)).await;
}
