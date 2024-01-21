use crate::model::JsonErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rust_axum_app::model::user::SearchedUser;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUser {
    pub id: i32,
    pub name: String,
}
impl From<SearchedUser> for JsonUser {
    fn from(searched_user: SearchedUser) -> Self {
        JsonUser {
            id: searched_user.id,
            name: searched_user.name,
        }
    }
}

pub enum UserError {
    NotFound,
    ServerError,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        match self {
            UserError::NotFound => {
                let json =
                    JsonErrorResponse::new(
                        vec!["ユーザー情報が見つかりませんでした。".to_string()],
                    );
                (StatusCode::NOT_FOUND, Json(json)).into_response()
            }
            UserError::ServerError => {
                let json = JsonErrorResponse::new(vec![
                    "ユーザー情報を検索中にエラーが発生しました。".to_string(),
                ]);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
            }
        }
    }
}
