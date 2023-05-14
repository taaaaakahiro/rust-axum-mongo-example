use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use mongodb::bson::doc;
use rust_axum_kernel::model::user::User;
use rust_axum_kernel::repository::user::UserRepository;

#[async_trait]
impl UserRepository for MongoDBRepositoryImpl {
    async fn get_user(&self, id: String) -> anyhow::Result<Option<User>> {
        let collection = self.db.0.collection::<UserDocument>("mountains");

        let filter = doc! {"_id": id};
        let mountain_doc = collection.find_one(filter, None).await?;
        match mountain_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }
}
