use crate::model::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, id: String) -> anyhow::Result<Option<User>>;
}
