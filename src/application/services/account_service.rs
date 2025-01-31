use std::sync::Arc;

use crate::{
    application::{
        entities::account_entity::AccountEntity,
        repositories::account_repository::AccountRepository,
    },
    core::http::failure::Failure,
};

#[async_trait::async_trait]
pub trait AccountService: Send + Sync {
    async fn create_account(&self, data: &AccountEntity) -> Result<AccountEntity, Failure>;
    async fn find_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Failure>;
    async fn find_by_id(&self, id: &str) -> Result<Option<AccountEntity>, Failure>;
}

pub struct AccountServiceImpl {
    pub account_repository: Arc<dyn AccountRepository>,
}

impl AccountServiceImpl {
    pub fn new(account_repository: Arc<dyn AccountRepository>) -> Self {
        AccountServiceImpl { account_repository }
    }
}

#[async_trait::async_trait]
impl AccountService for AccountServiceImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<AccountEntity>, Failure> {
        self.account_repository.find_by_email(email).await
    }

    async fn create_account(&self, data: &AccountEntity) -> Result<AccountEntity, Failure> {
        let account = self.account_repository.find_by_email(&data.email).await?;
        if account.is_some() {
            return Err(Failure::Conflict(
                "Account with this email already exists".to_string(),
            ));
        }

        self.account_repository.create(data).await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<AccountEntity>, Failure> {
        self.account_repository.find_by_id(id).await
    }
}
