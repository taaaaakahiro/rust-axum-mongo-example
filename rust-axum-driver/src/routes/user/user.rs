use axum::{http::StatusCode, response::IntoResponse, Json};
use rust_axum_kernel::model::user;

pub async fn get_user(
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
