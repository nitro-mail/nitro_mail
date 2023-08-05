use async_trait::async_trait;
use auto_impl::auto_impl;
use std::sync::Arc;

use utils::account::Account;
use utils::service::Service;
use utils::service_configuration::ServiceConfigurationResponse;

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
#[async_trait]
impl<T: Directory> Directory for Arc<T> {
    fn directory_name() -> &'static str
    where
        Self: Sized,
    {
        T::directory_name()
    }

    async fn load(config: Self::ServiceConfig) -> Result<Self, Self::ServiceError>
    where
        Self: Sized,
    {
        T::load(config).await.map(|t| Arc::new(t))
    }

    async fn get_account(&self, username: String) -> Result<Option<Account>, Self::ServiceError> {
        (**self).get_account(username).await
    }

    async fn login_account(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<Account>, Self::ServiceError> {
        (**self).login_account(username, password).await
    }

    async fn get_groups(&self) -> Result<Vec<String>, Self::ServiceError> {
        (**self).get_groups().await
    }

    async fn validate_config(
        &self,
        validate_config_request: ValidateDirectoryRequest,
    ) -> Result<ServiceConfigurationResponse, Self::ServiceError> {
        (**self).validate_config(validate_config_request).await
    }
}
