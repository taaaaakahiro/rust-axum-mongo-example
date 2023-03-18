use axum::{http::StatusCode, response::Html, response::IntoResponse, routing::get, Json, Router};
use rust_axum_driver::config::Config;
use rust_axum_kernel::model::user;
use std::net::SocketAddr;

#[tokio::main] // main関数を非同期関数にするために必要
async fn main() {
    // init
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();
    let config = Config::new();

    // db

    // router
    let app = Router::new()
        .route("/", get(handler))
        .route("/user", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<user::RequestUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = user::User {
        id: payload.id,
        name: payload.name,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
