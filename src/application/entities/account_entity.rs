use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::base_entity::BaseEntity;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub device_token: String,
}

impl AccountEntity {
    pub fn new(
        uuid: String,
        name: String,
        email: String,
        password: String,
        avatar: String,
        device_token: String,
    ) -> Self {
        AccountEntity {
            base: BaseEntity {
                id: Some(ObjectId::new()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            },
            uuid,
            name,
            email,
            password,
            device_token,
            avatar,
        }
    }
}
