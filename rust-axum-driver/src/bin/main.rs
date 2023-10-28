use rust_axum_driver::{
    config::Config,
    module::Modules,
    startup::{init_app, startup},
};
use std::sync::Arc;

#[tokio::main] //main関数を非同期関数にする
async fn main() -> anyhow::Result<()> {
    // init
    init_app();

    let cfg = Config::new();
    let modules = Modules::new().await;
    let _ = startup(&cfg, Arc::new(modules)).await;

    Ok(())
}
