use crate::model::Id;
use serde::Deserialize;

#[derive(Debug)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
}

impl User {
    pub fn new(id: Id<User>, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Deserialize)]
pub struct RequestUser {
    pub id: String,
    pub name: String,
}
