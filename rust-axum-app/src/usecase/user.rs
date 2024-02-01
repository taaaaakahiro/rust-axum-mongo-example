use crate::model::user::SearchedUser;
use rust_axum_adapter::modules::RepositoriesModuleExt;
use rust_axum_kernel::model::user::{UserFindException, UserGetException};
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

    pub async fn find_one(&self, id: String) -> Result<Option<SearchedUser>, UserGetException> {
        match id.try_into() {
            Ok(id) => match self.repositories.user_repository().find_one(id).await {
                Ok(user) => match user {
                    Some(user) => Ok(Some(user.into())),
                    None => Ok(None),
                },
                Err(_) => Err(UserGetException::new(ErrorCode::ServerError)),
            },
            Err(error_code) => Err(UserGetException::new(error_code)),
        }
    }

    pub async fn find(&self) -> Result<Vec<SearchedUser>, UserFindException> {
        match self.repositories.user_repository().find().await {
            Ok(users) => {
                let searched_users: Vec<SearchedUser> =
                    users.into_iter().map(|user| user.into()).collect();
                Ok(searched_users)
            }
            Err(err) => {
                let messages = vec![err.to_string()]; // Convert the error to a string message
                Err(UserFindException::new(ErrorCode::ServerError, messages))
            }
        }
    }
}
