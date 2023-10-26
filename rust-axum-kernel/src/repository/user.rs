use crate::model::user::User;
use crate::model::Id;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, id: Id<User>) -> anyhow::Result<Option<User>>;
}
