use crate::{persistence::mongodb::Db, repository::MongoDBRepositoryImpl};
use rust_axum_kernel::{model::user::User, repository::user::UserRepository};

//MEMO: DIP/依存性の注入
pub struct RepositoriesModule {
    user_repository: MongoDBRepositoryImpl<User>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;

    fn user_repository(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = MongoDBRepositoryImpl<User>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = MongoDBRepositoryImpl::new(db.clone());
        Self { user_repository }
    }
}
