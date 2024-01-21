use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use mongodb::bson::doc;
use rust_axum_kernel::model::Id;
use rust_axum_kernel::{model::user::User, repository::user::UserRepository};

#[async_trait]
impl UserRepository for MongoDBRepositoryImpl<User> {
    async fn get_user(&self, id: Id<User>) -> anyhow::Result<Option<User>> {
        let col = self.db.0.collection::<UserDocument>("users");
        let filter = doc! {"_id": id.value};
        let user_doc = col.find_one(filter, None).await?;
        tracing::info!("user_id ({})", &id.value);
        match user_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::persistence::mongodb::Db;
    use crate::repository::MongoDBRepositoryImpl;
    use rust_axum_kernel::model::user::User;
    use rust_axum_kernel::model::Id;
    use rust_axum_kernel::repository::user::UserRepository;

    #[tokio::test]
    async fn demo_test() {
        let user_id = 123; // i32 に変更
        let db = Db::new().await;
        let repo = MongoDBRepositoryImpl::new(db.clone());

        let result = repo.get_user(Id::<User>::new(user_id)).await;
        assert!(result.is_ok());
        let option = result.expect("failed to get user");
        assert!(option.is_none());
    }
}
