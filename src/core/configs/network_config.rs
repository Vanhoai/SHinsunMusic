use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NetworkConfig {
    pub host: String,
    pub port: String,
}
