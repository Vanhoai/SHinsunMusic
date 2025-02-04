use once_cell::sync::OnceCell;
use std::sync::Arc;

use super::database::init_database;
use crate::{
    adapters::secondary::{
        apis::{audio_api::AudioApiImpl, auth_api::AuthApiImpl},
        repositories::{
            account_repository::AccountRepositoryImpl, audio_repository::AudioRepositoryImpl,
        },
    },
    application::{
        domain::{
            account_domain::AccountDomain, audio_domain::AudioDomain, auth_domain::AuthDomain,
        },
        services::{
            account_service::AccountServiceImpl, audio_service::AudioServiceImpl,
            auth_service::AuthServiceImpl,
        },
    },
    core::jwt::service::JwtServiceImpl,
    state::AppState,
};

pub static AUTH_DOMAIN: OnceCell<Arc<AuthDomain>> = OnceCell::new();
pub static ACCOUNT_DOMAIN: OnceCell<Arc<AccountDomain>> = OnceCell::new();
pub static AUDIO_DOMAIN: OnceCell<Arc<AudioDomain>> = OnceCell::new();

pub async fn build_di() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    // Utils
    let jwt_service = Arc::new(JwtServiceImpl::new());
    let database = Arc::new(init_database().await);

    // Collections
    let accounts = Arc::new(database.collection("accounts"));
    let audios = Arc::new(database.collection("audios"));

    // Repositories
    let account_repository = Arc::new(AccountRepositoryImpl::new(accounts.clone()));
    let audio_repository = Arc::new(AudioRepositoryImpl::new(audios.clone()));

    // Api & Service
    // auth
    let auth_api = Arc::new(AuthApiImpl::new());
    let auth_service = Arc::new(AuthServiceImpl::new(account_repository.clone()));

    // account
    let account_service = Arc::new(AccountServiceImpl::new(account_repository.clone()));

    // audio
    let audio_api = Arc::new(AudioApiImpl::new());
    let audio_service = Arc::new(AudioServiceImpl::new(audio_repository.clone()));

    // Domain Logic
    // auth
    let auth_domain = Arc::new(AuthDomain::new(
        auth_service.clone(),
        account_service.clone(),
        auth_api.clone(),
        jwt_service.clone(),
    ));

    // account
    let account_domain = Arc::new(AccountDomain::new(account_service.clone()));

    // audio
    let audio_domain = Arc::new(AudioDomain::new(audio_service.clone(), audio_api.clone()));

    AUTH_DOMAIN
        .set(auth_domain)
        .map_err(|_| "Failed to set AUTH_HANDLER")?;

    ACCOUNT_DOMAIN
        .set(account_domain)
        .map_err(|_| "Failed to set ACCOUNT_HANDLER")?;

    AUDIO_DOMAIN
        .set(audio_domain)
        .map_err(|_| "Failed to set AUDIO_HANDLER")?;

    let state = Arc::new(AppState {
        jwt_service: jwt_service.clone(),
    });

    Ok(state)
}

pub fn auth_domain() -> Arc<AuthDomain> {
    AUTH_DOMAIN
        .get()
        .expect("AUTH DOMAIN not initialized")
        .clone()
}

pub fn account_domain() -> Arc<AccountDomain> {
    ACCOUNT_DOMAIN
        .get()
        .expect("ACCOUNT DOMAIN not initialized")
        .clone()
}

pub fn audio_domain() -> Arc<AudioDomain> {
    AUDIO_DOMAIN
        .get()
        .expect("AUDIO DOMAIN not initialized")
        .clone()
}
