use crate::persistence::mongodb::Db;

pub struct RepositoriesModule {}

pub trait RepositoriesModuleExt {}

impl RepositoriesModuleExt for RepositoriesModule {}

impl RepositoriesModule {
    pub fn new(_db: Db) -> Self {
        Self {}
    }
}
