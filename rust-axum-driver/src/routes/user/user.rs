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

pub async fn find_one(
    Path(user_id): Path<String>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let res = modules.user_use_case().find_one(user_id).await;
    match res {
        Ok(su) => {
            return match su {
                Some(su) => {
                    tracing::info!("Succeeded to get user by id ({}).", &su.id);

                    let json: JsonUser = su.into();
                    Ok((StatusCode::OK, Json(json)))
                }
                None => {
                    tracing::info!("Succeeded to get user by id (None).");
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

pub async fn find(Extension(modules): Extension<Arc<Modules>>) -> impl IntoResponse {
    let res = modules.user_use_case().find().await;

    match res {
        Ok(searched_users) => {
            let json_users: Vec<JsonUser> =
                searched_users.into_iter().map(|su| su.into()).collect();
            Ok((StatusCode::OK, Json(json_users)))
        }
        Err(get_ex) => {
            error!("{:?}", get_ex);
            let error_response: Result<(), UserError> = match &get_ex.error_code {
                ErrorCode::InvalidId => Err(UserError::NotFound),
                _ => Err(UserError::ServerError),
            };

            Err(error_response.into_response())
        }
    }
}
