use crate::{
    application::apis::auth_api::{AuthApi, VerifyIdTokenResponse},
    core::http::failure::Failure,
};

pub struct AuthApiImpl {}

impl AuthApiImpl {
    pub fn new() -> Self {
        AuthApiImpl {}
    }
}

#[async_trait::async_trait]
impl AuthApi for AuthApiImpl {
    async fn verify_id_token(&self, _: String) -> Result<VerifyIdTokenResponse, Failure> {
        todo!()
    }
}
