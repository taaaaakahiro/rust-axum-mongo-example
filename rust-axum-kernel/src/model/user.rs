use crate::model::{ErrorCode, Id};
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

#[derive(Debug)]
pub struct UserGetException {
    pub error_code: ErrorCode,
}

impl UserGetException {
    pub fn new(error_code: ErrorCode) -> Self {
        Self { error_code }
    }
}
