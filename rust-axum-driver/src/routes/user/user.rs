use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Json};
use rust_axum_kernel::model::user::{RequestUser, User};

pub async fn post_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<RequestUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: payload.id,
        name: payload.name,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(Path(user_id): Path<String>) -> impl IntoResponse {
    // todo: usecase
    let user = User {
        id: user_id.to_owned(),
        name: format!("name{}", user_id.to_owned()),
    };

    (StatusCode::OK, Json(user))
}
