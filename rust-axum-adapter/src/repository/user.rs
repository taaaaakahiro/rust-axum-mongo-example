use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
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

    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let col = self.db.0.collection::<UserDocument>("users");
        let filter = doc! {}; // 空のフィルタは全てのドキュメントを取得します
        let mut cursor = col.find(filter, None).await?;
        let mut users = Vec::new();

        // カーソルをイテレートして全てのユーザーを収集します
        while let Some(user_doc) = cursor.try_next().await? {
            users.push(user_doc.try_into()?);
        }
        Ok(users)
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn it_works() {
        assert_eq!(1 + 2, 3);
    }
    #[tokio::test]
    async fn demo_test() {
        assert_eq!(1 + 2, 3); //仮
                              //TODO: 後で
                              // let db = Db::new().await;
                              // let id = "63b5700f67a2592b8942f971";
                              // let got = MongoDBRepositoryImpl::assert_eq!(2 + 2, got.try_into());
    }
}
