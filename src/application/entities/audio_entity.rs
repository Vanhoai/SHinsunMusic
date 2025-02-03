use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::base_entity::BaseEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub url: String,
    pub name: String,
    pub thumbnail: String,
}

impl AudioEntity {
    pub fn new(url: String, name: String, thumbnail: String) -> Self {
        AudioEntity {
            base: BaseEntity {
                id: Some(ObjectId::new()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            url,
            name,
            thumbnail,
        }
    }
}
