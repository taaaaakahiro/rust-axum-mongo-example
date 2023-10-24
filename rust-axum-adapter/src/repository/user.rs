use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use mongodb::bson::doc;
use rust_axum_kernel::model::user::User;
use rust_axum_kernel::repository::user::UserRepository;

#[async_trait]
impl UserRepository for MongoDBRepositoryImpl<User> {
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

#[cfg(test)]
mod test {
    //TODO: 後で
    use super::MongoDBRepositoryImpl;
    use crate::persistence::mongodb::Db;
    // use rust_axum_kernel::model::Id;
    // use ulid::Ulid;

    // #[test]
    // #[ignore]
    #[tokio::test]
    async fn demo_testcase() {
        //TODO: 後で
        // let db = Db::new().await;
        // let repo: MongoDBRepositoryImpl = MongoDBRepositoryImpl::new(db);
        // let id = Ulid::new();
        // let got = repo.get_user(&Id::new(id));

        assert_eq!(2 + 2, 4);
        assert_eq!(1 + 2, 3);
    }
}
