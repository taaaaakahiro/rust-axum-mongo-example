use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use mongodb::bson::doc;
use rust_axum_kernel::model::user::User;
use rust_axum_kernel::repository::user::UserRepository;

#[async_trait]
impl UserRepository for MongoDBRepositoryImpl<User> {
    async fn get_user(&self, id: String) -> anyhow::Result<Option<User>> {
        let col = self.db.0.collection::<UserDocument>("user");

        let filter = doc! {"_id": id};
        let mountain_doc = col.find_one(filter, None).await?;
        match mountain_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    //TODO: 後で
    // use super::MongoDBRepositoryImpl;
    // use crate::persistence::mongodb::Db;
    // use crate::repository::user::UserRepository;
    // use rust_axum_kernel::model::user::User;
    // use rust_axum_kernel::model::Id;

    #[tokio::test]
    async  fn it_works(){
        assert_eq!(1 + 2, 3);
    }
    #[test]
     fn demo_test() {
        assert_eq!(1 + 2, 3); //仮
        //TODO: 後で
        // let db = Db::new().await;
        // let id = "63b5700f67a2592b8942f971";
        // let got = MongoDBRepositoryImpl::assert_eq!(2 + 2, got.try_into());
    }
}
