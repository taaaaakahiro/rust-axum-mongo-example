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
    #[ignore]
    async fn test_find_one() {
        struct Test {
            _name: &'static str,
            user_id: i32,
            want: Option<User>,
        }

        let tests = vec![
            Test {
                _name: "ok1",
                user_id: 1,
                want: Some(User {
                    id: Id::new(1),
                    name: "Hoge".to_string(),
                }),
            },
            Test {
                _name: "ok2",
                user_id: 2,
                want: Some(User {
                    id: Id::new(2),
                    name: "Fuga".to_string(),
                }),
            },
            Test {
                _name: "ng",
                user_id: 999,
                want: None, // Expecting None for a non-existent user
            },
        ];

        for test in tests {
            let result = get_repo()
                .await
                .find_one(Id::<User>::new(test.user_id))
                .await;

            match result {
                Ok(Some(got)) => {
                    let want = test.want.unwrap();
                    assert_eq!(got.id.value, want.id.value);
                    assert_eq!(got.name, want.name);
                }
                Ok(None) => {
                    assert!(test.want.is_none(), "Expected Some, got None");
                }
                Err(_) => {
                    panic!("Failed to get user");
                }
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_find() {
        struct Test {
            _name: &'static str,
            want: Vec<User>,
        }
        let tests = vec![Test {
            _name: "ok",
            want: vec![
                User {
                    id: Id::new(1),
                    name: "Hoge".to_string(),
                },
                User {
                    id: Id::new(2),
                    name: "Fuga".to_string(),
                },
            ],
        }];

        for test in tests {
            let got = get_repo().await.find().await.expect("failed to get users");
            assert_eq!(got.len(), test.want.len(), "Lengths do not match");

            for (got_user, expected_user) in got.iter().zip(test.want.iter()) {
                assert_eq!(got_user.id.value, expected_user.id.value);
                assert_eq!(got_user.name, expected_user.name);
            }
        }
    }

    async fn get_repo() -> MongoDBRepositoryImpl<User> {
        let db = Db::new().await;
        MongoDBRepositoryImpl::new(db.clone())
    }
}
