use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::Arc;

use ahash::{HashMap, HashMapExt};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use directories::directory_type::Directory;
use directories::ValidateDirectoryRequest;
use utils::account::{Account, EmailAddress};
use utils::service::Service;
use utils::service_configuration::ServiceConfigurationResponse;
use utils::Config;

pub mod shared_constants {
    include!("../../../tests/shared_constants.rs");
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailBox {
    pub emails: Vec<Vec<u8>>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TestAccount {
    pub account: Account,
    pub email_addresses: Vec<EmailAddress>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub accounts: Vec<TestAccount>,
}

impl Default for TestConfig {
    fn default() -> Self {
        let mut config = Vec::new();
        config.push(TestAccount {
            account: Account {
                username: shared_constants::TEST_USER_NAME.to_string(),
                account_type: Default::default(),
            },
            email_addresses: vec![],
        });
        Self { accounts: config }
    }
}
impl Config for TestConfig {
    fn config_header() -> &'static str
    where
        Self: Sized,
    {
        "test_config"
    }

    fn config_name() -> &'static str
    where
        Self: Sized,
    {
        "test_config.toml"
    }
}
#[derive(Debug)]
pub struct TestDirectoryInner {
    pub accounts: RwLock<HashSet<TestAccount>>,
    pub mail_boxes: RwLock<HashMap<Uuid, MailBox>>,
}
#[derive(Debug, Clone)]
pub struct TestDirectory(Arc<TestDirectoryInner>);
impl Service for TestDirectory {
    type ServiceConfig = TestConfig;
    type ServiceError = Infallible;
}
#[async_trait]
impl Directory for TestDirectory {
    fn directory_name() -> &'static str
    where
        Self: Sized,
    {
        "test_directory"
    }

    async fn load(config: Self::ServiceConfig) -> Result<Self, Self::ServiceError>
    where
        Self: Sized,
    {
        let mut accounts = HashSet::new();
        for account in config.accounts {
            accounts.insert(account.clone());
        }
        Ok(Self(Arc::new(TestDirectoryInner {
            accounts: RwLock::new(accounts),
            mail_boxes: RwLock::new(HashMap::new()),
        })))
    }

    async fn get_account(&self, username: String) -> Result<Option<Account>, Self::ServiceError> {
        let accounts = self.0.accounts.read();
        Ok(accounts
            .iter()
            .find(|a| a.account.username == username)
            .cloned()
            .map(|a| a.account))
    }

    async fn login_account(
        &self,
        username: String,
        _: String,
    ) -> Result<Option<Account>, Self::ServiceError> {
        let accounts = self.0.accounts.read();
        Ok(accounts
            .iter()
            .find(|a| a.account.username == username)
            .cloned()
            .map(|a| a.account))
    }

    async fn get_groups(&self) -> Result<Vec<String>, Self::ServiceError> {
        todo!()
    }

    async fn validate_config(
        &self,
        validate_config_request: ValidateDirectoryRequest,
    ) -> Result<ServiceConfigurationResponse, Self::ServiceError> {
        todo!()
    }
}
