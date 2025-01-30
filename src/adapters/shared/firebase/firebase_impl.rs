use firebase_auth_sdk::FireAuth;
use std::sync::Arc;

use super::claims::IdTokenClaims;
use super::firebase_app::{FirebaseApp, FIREBASE_APP};
use crate::core::{configs::app_config::APP_CONFIG, http::failure::Failure};

pub struct FirebaseAppImpl {
    pub firebase_auth: Arc<FireAuth>,
}

pub async fn initial() -> Result<(), Box<dyn std::error::Error>> {
    let firebase_auth = Arc::new(FireAuth::new(APP_CONFIG.key.web_api_key.clone()));

    let firebase_app = Arc::new(FirebaseAppImpl {
        firebase_auth: firebase_auth.clone(),
    });

    FIREBASE_APP
        .set(firebase_app)
        .map_err(|_| "Failed to set firebase")?;

    Ok(())
}

#[async_trait::async_trait]
impl FirebaseApp for FirebaseAppImpl {
    async fn verify_auth_token(&self, id_token: &str) -> Result<IdTokenClaims, Failure> {
        let claims = self
            .firebase_auth
            .verify_id_token(id_token)
            .await
            .map_err(|e| Failure::Unauthorized(e.to_string()))?;

        Ok(IdTokenClaims {
            exp: claims.exp,
            iat: claims.iat,
            iss: claims.iss,
            sub: claims.sub,
            auth_time: claims.auth_time,
        })
    }
}
