use std::sync::Arc;

use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

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

    async fn find_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Failure> {
        let query = doc! { "email": email };

        let account = self
            .collection
            .find_one(query)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(account)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<AccountEntity>, Failure> {
        let query = doc! { "_id": ObjectId::parse_str(id).unwrap() };

        let account = self
            .collection
            .find_one(query)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(account)
    }
}
