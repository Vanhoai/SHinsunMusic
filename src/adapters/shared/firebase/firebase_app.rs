use once_cell::sync::OnceCell;
use std::sync::Arc;

use crate::core::http::failure::Failure;

use super::claims::IdTokenClaims;

pub static FIREBASE_APP: OnceCell<Arc<dyn FirebaseApp>> = OnceCell::new();

#[async_trait::async_trait]
pub trait FirebaseApp: Send + Sync {
    async fn verify_auth_token(&self, id_token: &str) -> Result<IdTokenClaims, Failure>;
}

pub fn firebase() -> Arc<dyn FirebaseApp> {
    FIREBASE_APP
        .get()
        .expect("FIREBASE_APP not initialized")
        .clone()
}
