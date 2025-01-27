use crate::application::repositories::account_repository::AccountRepository;

pub struct AccountRepositoryImpl {}

impl AccountRepositoryImpl {
    pub fn new() -> Self {
        AccountRepositoryImpl {}
    }
}

impl AccountRepository for AccountRepositoryImpl {}
