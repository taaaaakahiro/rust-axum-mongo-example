use crate::model::user::User;
use crate::model::Id;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: Id<User>) -> anyhow::Result<Option<User>>;
    async fn find(&self) -> anyhow::Result<Vec<User>>;
}
