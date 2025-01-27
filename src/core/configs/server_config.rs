use serde::Deserialize;

use super::network_config::NetworkConfig;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(flatten)]
    pub network: NetworkConfig,
}
