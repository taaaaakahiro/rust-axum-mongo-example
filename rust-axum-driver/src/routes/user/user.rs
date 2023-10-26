use crate::model::user::{JsonUser, UserError};
use crate::module::{Modules, ModulesExt};
use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use rust_axum_kernel::model::user::RequestUser;
use rust_axum_kernel::model::ErrorCode;
use std::sync::Arc;
use tracing::error;

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

pub async fn get_user(
    Path(user_id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let res = modules.user_use_case().get_user(user_id).await;
    match res {
        Ok(sm) => {
            return match sm {
                Some(sm) => {
                    tracing::info!("Succeeded to get user by id ({}).", &sm.id);

                    let json: JsonUser = sm.into();
                    Ok((StatusCode::OK, Json(json)))
                }
                None => {
                    tracing::info!("Succeeded to get mountain by id (None).");
                    Err(UserError::NotFound)
                }
            }
        }
        Err(get_ex) => {
            error!("{:?}", get_ex);
            if get_ex.error_code == ErrorCode::InvalidId {
                Err(UserError::NotFound)
            } else {
                Err(UserError::ServerError)
            }
        }
    }
}
