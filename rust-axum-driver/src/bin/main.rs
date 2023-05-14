use rust_axum_driver::{
    config::Config,
    startup::{init_app, startup},
};

#[tokio::main] //main関数を非同期関数にする
async fn main() -> anyhow::Result<()> {
    // init
    init_app();

    let cfg = Config::new();
    let _ = startup(&cfg).await;

    Ok(())
}
