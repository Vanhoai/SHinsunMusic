use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    Collection,
};
use std::{sync::Arc, vec};

use crate::{
    application::{
        entities::audio_entity::AudioEntity, repositories::audio_repository::AudioRepository,
        usecases::audio_usecases::SearchAudioResponse,
    },
    core::{
        base::base_query::SearchQuery,
        http::{failure::Failure, response::Meta},
    },
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

    async fn update(&self, data: &AudioEntity) -> Result<AudioEntity, Failure> {
        let id = data.base.id.unwrap();
        let query = doc! { "_id": id };

        let update = doc! {
            "$set": doc! {
                "name": data.name.as_str(),
                "ytId": data.yt_id.as_str(),
                "audio": data.audio.as_str(),
                "thumbnail": data.thumbnail.as_str(),
            }
        };

        self.collection
            .update_one(query, update)
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

    async fn find_by_yt_id(&self, yt_id: &str) -> Result<Option<AudioEntity>, Failure> {
        let query = doc! { "ytId": yt_id };
        let audio = self
            .collection
            .find_one(query)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        Ok(audio)
    }

    /// FIXME: Write a common function for pagination
    async fn search(&self, query: &SearchQuery) -> Result<SearchAudioResponse, Failure> {
        let page = query.base_query.page;
        let page_size = query.base_query.page_size;

        let skip = ((page - 1) * page_size) as i64;

        let pipeline = vec![
            doc! {
                "$match": {
                    "name": {
                        "$regex": query.search.as_str(),
                        "$options": "i"
                    }
                }
            },
            doc! {
                "$skip": skip,
            },
            doc! {
                "$limit": page_size as i64,
            },
        ];

        let total_record =
            self.collection
                .count_documents(doc! {})
                .await
                .map_err(|e| Failure::DatabaseError(e.to_string()))? as usize;

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?;

        let mut audios = vec![];
        while let Some(doc) = cursor
            .try_next()
            .await
            .map_err(|e| Failure::DatabaseError(e.to_string()))?
        {
            let entity: AudioEntity =
                bson::from_document(doc).map_err(|e| Failure::DatabaseError(e.to_string()))?;

            audios.push(entity);
        }

        let total_page = (total_record as f64 / page_size as f64).ceil() as usize;

        let meta = Meta {
            page,
            page_size,
            total_page,
            total_record,
        };

        Ok(SearchAudioResponse { meta, audios })
    }
}
