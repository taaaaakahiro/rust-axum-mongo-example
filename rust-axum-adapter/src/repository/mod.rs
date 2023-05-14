use crate::persistence::mongodb::Db;

pub mod health_check;
pub mod user;

pub struct MongoDBRepositoryImpl {
    db: Db,
}

impl MongoDBRepositoryImpl {
    pub fn new(db: Db) -> Self {
        Self { db: db }
    }
}
