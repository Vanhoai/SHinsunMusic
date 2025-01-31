use std::sync::Arc;

use crate::{
    application::{
        entities::account_entity::AccountEntity, services::account_service::AccountService,
        usecases::account_usecases::ManageAccountUseCases,
    },
    core::{helpers::validate_helper, http::failure::Failure},
};

pub struct AccountDomain {
    pub account_service: Arc<dyn AccountService>,
}

impl AccountDomain {
    pub fn new(account_service: Arc<dyn AccountService>) -> Self {
        AccountDomain { account_service }
    }
}

#[async_trait::async_trait]
impl ManageAccountUseCases for AccountDomain {
    async fn find_profile_with_id(&self, id: &str) -> Result<AccountEntity, Failure> {
        validate_helper::is_valid_object_id(id)?;

        let account = self.account_service.find_by_id(id).await?;
        if account.is_none() {
            return Err(Failure::NotFound(
                "Account with id not found in database".to_string(),
            ));
        }

        Ok(account.unwrap())
    }

    async fn update_profile(&self, _: &AccountEntity) -> Result<AccountEntity, Failure> {
        todo!()
    }
}
