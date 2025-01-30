use std::sync::Arc;

use config::{Config, Environment};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::{key_config::KeyConfig, server_config::ServerConfig};

#[derive(Serialize, Deserialize)]
pub struct AppCommonConfig {
    pub mode: String,
    pub api_version: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub common: AppCommonConfig,
    pub server: ServerConfig,
    pub key: KeyConfig,
}

#[derive(Deserialize)]
pub struct ProductionConfig {
    #[serde(flatten)]
    pub common: AppCommonConfig,
    pub server: ServerConfig,
    pub key: KeyConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let builder = Config::builder().add_source(Environment::default());
        let config = builder.build().expect("Please config env for application");

        let deserialized = config
            .try_deserialize::<ProductionConfig>()
            .expect("Please add env production for application");

        AppConfig::new(deserialized.common, deserialized.server, deserialized.key)
    }

    pub fn new(common: AppCommonConfig, server: ServerConfig, key: KeyConfig) -> Self {
        AppConfig {
            common,
            server,
            key,
        }
    }
}

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| Arc::new(AppConfig::from_env()));
