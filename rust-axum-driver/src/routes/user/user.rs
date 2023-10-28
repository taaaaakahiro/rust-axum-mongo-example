use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Json};
use rust_axum_kernel::model::user::RequestUser;

pub async fn post_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(_payload): Json<RequestUser>,
) -> impl IntoResponse {
    // insert your application logic here
    //TODO: 後で
    tracing::info!("success!");
    (StatusCode::CREATED, Json("success"))
}

pub async fn get_user(Path(_user_id): Path<String>) -> impl IntoResponse {
    //TODO: 後で
    tracing::info!("success!");
    (StatusCode::OK, Json("success"))
}
