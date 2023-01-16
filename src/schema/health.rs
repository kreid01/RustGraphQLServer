use async_graphql::{Object, Result};

use crate::prisma;
use prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation;
use prisma::user;

#[derive(Default)]
pub struct PostQuery;
#[Object]
impl PostQuery {

    async fn post(&self) ->  String { 
      let client = prisma::new_client().await.unwrap();
      let user = client.user().create(
        "abcdef".to_string(),
        "Kieran".to_string(),
        vec![]
       ).exec().await;

       match user {
        Ok(_user) => ("User created").to_string(),
        Err(error) if error.is_prisma_error::<UniqueKeyViolation>() =>
            ("Unique key violated").to_string(),
        Err(_error) => ("Other error occurred").to_string()
       }
    }

    async fn get(&self) -> Result<Vec<user>> {
      let client = prisma::new_client().await.unwrap();
        client.user().find_many(vec![]).exec().await.unwrap()
    }
}