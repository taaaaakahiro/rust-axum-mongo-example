use crate::model::user::SearchedUser;
use rust_axum_adapter::modules::RepositoriesModuleExt;
use rust_axum_kernel::model::user::{User, UserGetException};
use rust_axum_kernel::model::ErrorCode;
use rust_axum_kernel::repository::user::UserRepository;
use std::sync::Arc;

pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get_user(&self, id: String) -> Result<Option<SearchedUser>, UserGetException> {
        match id.try_into() {
            Ok(id) => match self.repositories.user_repository().get_user(id).await {
                Ok(user) => match user {
                    Some(user) => Ok(Some(user.into())),
                    None => Ok(None),
                },
                Err(_) => Err(UserGetException::new(ErrorCode::ServerError)),
            },
            Err(error_code) => Err(UserGetException::new(error_code)),
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserGetException> {
        match self.repositories.user_repository().list_users().await {
            Ok(users) => Ok(users),
            Err(_) => Err(UserGetException::new(ErrorCode::ServerError)),
        }
    }
}
