use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use std::sync::Arc;

use crate::{
    application::{
        entities::audio_entity::AudioEntity, repositories::audio_repository::AudioRepository,
    },
    core::http::failure::Failure,
};

pub struct AudioRepositoryImpl {
    pub collection: Arc<Collection<AudioEntity>>,
}

impl AudioRepositoryImpl {
    pub fn new(collection: Arc<Collection<AudioEntity>>) -> Self {
        return AudioRepositoryImpl { collection };
    }
}

#[async_trait::async_trait]
impl AudioRepository for AudioRepositoryImpl {
    async fn create(&self, data: &AudioEntity) -> Result<AudioEntity, Failure> {
        self.collection
            .insert_one(data)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(data.clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<AudioEntity>, Failure> {
        let query = doc! { "_id": ObjectId::parse_str(id).unwrap() };

        let account = self
            .collection
            .find_one(query)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(account)
    }
}
