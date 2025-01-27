use std::sync::Arc;

use once_cell::sync::OnceCell;

use crate::{
    adapters::secondary::{
        apis::auth_api::AuthApiImpl, repositories::account_repository::AccountRepositoryImpl,
    },
    application::{domain::auth_domain::AuthDomain, services::auth_service::AuthServiceImpl},
};

pub static AUTH_DOMAIN: OnceCell<Arc<AuthDomain>> = OnceCell::new();

pub async fn build_di() -> Result<(), Box<dyn std::error::Error>> {
    let account_repository = Arc::new(AccountRepositoryImpl::new());

    let auth_api = Arc::new(AuthApiImpl::new());
    let auth_service = Arc::new(AuthServiceImpl::new(account_repository.clone()));

    let auth_handler = Arc::new(AuthDomain::new(auth_service.clone(), auth_api.clone()));

    AUTH_DOMAIN
        .set(auth_handler)
        .map_err(|_| "Failed to set AUTH_HANDLER")?;
    Ok(())
}

pub fn auth_domain() -> Arc<AuthDomain> {
    AUTH_DOMAIN
        .get()
        .expect("AUTH DOMAIN not initialized")
        .clone()
}
