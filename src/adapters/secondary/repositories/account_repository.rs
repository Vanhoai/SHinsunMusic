use std::sync::Arc;

use mongodb::{bson::doc, Collection};

use crate::{
    application::{
        entities::account_entity::AccountEntity,
        repositories::account_repository::AccountRepository,
    },
    core::http::failure::Failure,
};

pub struct AccountRepositoryImpl {
    pub collection: Arc<Collection<AccountEntity>>,
}

impl AccountRepositoryImpl {
    pub fn new(collection: Arc<Collection<AccountEntity>>) -> Self {
        AccountRepositoryImpl { collection }
    }
}

#[async_trait::async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn create(&self, data: &AccountEntity) -> Result<AccountEntity, Failure> {
        self.collection
            .insert_one(data)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(data.clone())
    }

    async fn find_by_email(&self, email: &str) -> Result<AccountEntity, Failure> {
        let query = doc! { "email": email };

        let account = self
            .collection
            .find_one(query)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        match account {
            Some(account) => Ok(account),
            None => Err(Failure::NotFound(
                "Can not find account with given filter".to_string(),
            )),
        }
    }
}
