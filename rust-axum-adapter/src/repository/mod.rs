use crate::persistence::mongodb::Db;
use std::marker::PhantomData;

pub mod health_check;
pub mod user;

pub struct MongoDBRepositoryImpl<T> {
    db: Db,
    _marker: PhantomData<T>,
}

impl<T> MongoDBRepositoryImpl<T> {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }
}
