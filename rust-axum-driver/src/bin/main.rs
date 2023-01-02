use axum::{
    response::Html, routing::{get, post}, Router, Json,
    http::StatusCode,
    response::IntoResponse,
};
use std::{
    net::SocketAddr,
};
use serde::{Deserialize, Serialize};
use rust_axum_driver::config::load_confg;

#[tokio::main]  // main関数を非同期関数にするために必要
async fn main() {
    // init
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();

    // load env
    // let config = match envy::from_env::<Config>() {
    //     Ok(val) => val,
    //     Err(err) => {
    //         println!("{}", err);
    //         process::exit(1);
    //     }
    // };

    let config = load_confg();


    // db
    print!("{}", "s");

    



    // router
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(create_user));

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


async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
        num: 4444,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    num: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

