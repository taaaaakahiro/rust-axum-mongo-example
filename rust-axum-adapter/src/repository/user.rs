use crate::{model::user::UserDocument, repository::MongoDBRepositoryImpl};
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use rust_axum_kernel::model::Id;
use rust_axum_kernel::{model::user::User, repository::user::UserRepository};

#[async_trait]
impl UserRepository for MongoDBRepositoryImpl<User> {
    async fn find_one(&self, id: Id<User>) -> anyhow::Result<Option<User>> {
        let col = self.db.0.collection::<UserDocument>("users");
        let filter = doc! {"_id": id.value};
        let user_doc = col.find_one(filter, None).await?;
        tracing::info!("user_id ({})", &id.value);
        match user_doc {
            Some(md) => Ok(Some(md.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find(&self) -> anyhow::Result<Vec<User>> {
        let col = self.db.0.collection::<UserDocument>("users");
        let options = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cursor = col.find(None, options).await?;
        let mut users = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user: User = document.try_into()?;
                    users.push(user);
                }
                Err(err) => {
                    eprintln!("Error retrieving user document: {:?}", err);
                }
            }
        }

        Ok(users)
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
    async fn test_find_one() {
        let db = Db::new().await;
        let repo = MongoDBRepositoryImpl::new(db.clone());

        let id = 1;
        let result = repo.find_one(Id::<User>::new(id)).await;
        assert!(result.is_ok());
        let option = result.expect("failed to get user");
        assert!(option.is_some());
        let got = option.unwrap();
        assert_eq!(&got.name, "Hoge");
    }

    #[tokio::test]
    async fn test_find() {
        let db = Db::new().await;
        let repo = MongoDBRepositoryImpl::new(db.clone());
        let result = repo.find().await;

        assert!(result.is_ok());
        let users = result.expect("Failed to get users");
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "Hoge");
        assert_eq!(users[1].name, "Fuga");
    }
}
