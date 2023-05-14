use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RequestUser {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
