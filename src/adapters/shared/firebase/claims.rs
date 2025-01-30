use serde::{Deserialize, Serialize};

// The firebase ID token claims
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdTokenClaims {
    pub exp: u64,
    pub iat: u64,
    pub iss: String,
    pub sub: String,
    pub auth_time: u64,
}
