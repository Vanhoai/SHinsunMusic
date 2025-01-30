use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JwtConfig {
    pub key_type: String,
    pub access_exp: String,
    pub refresh_exp: String,
}
