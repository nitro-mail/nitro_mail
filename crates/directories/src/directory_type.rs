use async_trait::async_trait;

use utils::account::Account;
use utils::service::Service;
use utils::service_configuration::ServiceConfigurationResponse;
use utils::Config;

use crate::ValidateDirectoryRequest;

#[async_trait]
pub trait Directory: Service {
    fn directory_name() -> &'static str
    where
        Self: Sized;

    async fn load(config: Self::ServiceConfig) -> Result<Self, Self::ServiceError>
    where
        Self: Sized;
    async fn get_account(&self, username: String) -> Result<Option<Account>, Self::ServiceError>;

    async fn login_account(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<Account>, Self::ServiceError>;

    async fn get_groups(&self) -> Result<Vec<String>, Self::ServiceError>;

    async fn validate_config(
        &self,
        validate_config_request: ValidateDirectoryRequest,
    ) -> Result<ServiceConfigurationResponse, Self::ServiceError>;
}
