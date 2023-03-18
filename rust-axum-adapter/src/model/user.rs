use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDocument {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
}
