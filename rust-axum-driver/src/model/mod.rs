pub mod user;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorResponse {
    messages: Vec<String>,
}

impl JsonErrorResponse {
    pub(crate) fn new(messages: Vec<String>) -> Self {
        Self { messages }
    }
}
