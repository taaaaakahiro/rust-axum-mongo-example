use crate::persistence::mongodb::Db;
use std::marker::PhantomData;

pub mod health_check;

pub struct MongoDBRepositoryImpl<T> {
    _db: Db,
    _marker: PhantomData<T>,
}

impl<T> MongoDBRepositoryImpl<T> {
    pub fn new(db: Db) -> Self {
        Self {
            _db: db,
            _marker: PhantomData,
        }
    }
}
