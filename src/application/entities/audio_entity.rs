use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::base_entity::BaseEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub yt_id: String,
    pub audio: String,
    pub thumbnail: String,
}

impl AudioEntity {
    pub fn new(yt_id: String) -> Self {
        AudioEntity {
            base: BaseEntity {
                id: Some(ObjectId::new()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            yt_id,
            audio: "".to_string(),
            name: "".to_string(),
            thumbnail: "".to_string(),
        }
    }

    pub fn new_full(name: String, yt_id: String, audio: String, thumbnail: String) -> Self {
        AudioEntity {
            base: BaseEntity {
                id: Some(ObjectId::new()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            yt_id,
            audio,
            name,
            thumbnail,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioResponse {
    pub id: String,
    pub name: String,
    pub yt_id: String,
    pub audio: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AudioEntity> for AudioResponse {
    fn from(account: AudioEntity) -> Self {
        AudioResponse {
            id: account.base.id.unwrap().to_string(),
            name: account.name,
            yt_id: account.yt_id,
            audio: account.audio,
            thumbnail: account.thumbnail,
            created_at: account.base.created_at,
            updated_at: account.base.updated_at,
        }
    }
}

impl AudioResponse {
    pub fn array(audios: Vec<AudioEntity>) -> Vec<AudioResponse> {
        audios.into_iter().map(AudioResponse::from).collect()
    }
}
