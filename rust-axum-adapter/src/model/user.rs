use rust_axum_kernel::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDocument {
    #[serde(rename = "_id")]
    pub id: i32,
    pub name: String,
}

impl TryFrom<UserDocument> for User {
    type Error = anyhow::Error;
    fn try_from(user_doc: UserDocument) -> Result<Self, Self::Error> {
        let user_id = user_doc.id.into();

        Ok(User::new(user_id, user_doc.name))
    }
}
