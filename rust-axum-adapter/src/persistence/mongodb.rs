use mongodb::{Client, Database};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Database>);

impl Db {
    pub async fn new() -> Db {
        let uri = env::var("DATABASE_URL").expect("DATABASE_URL is undefined");
        let client = Client::with_uri_str(&uri)
            .await
            .expect("failed to connect to MongoDB");
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is undefined");
        let db = client.database(&db_name);
        Db(Arc::new(db))
    }
}
