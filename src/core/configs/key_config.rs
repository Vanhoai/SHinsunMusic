use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub web_api_key: String,
    pub skip_number: String,
}
