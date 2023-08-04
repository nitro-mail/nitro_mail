use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use thiserror::Error;

use directories::directory_type::Directory;
use directories::ValidateDirectoryRequest;
use utils::account::Account;
use utils::service::Service;
use utils::service_configuration::ServiceConfigurationResponse;

use crate::database_config::DatabaseConfig;

#[async_trait]
pub trait DatabaseDirectoryTrait: Send + Sync + 'static {
    async fn connect(connection_options: ConnectOptions) -> Result<Self, Error>
    where
        Self: Sized;

    fn name() -> &'static str
    where
        Self: Sized;
}
#[async_trait]
impl DatabaseDirectoryTrait for DatabaseConnection {
    async fn connect(connection_options: ConnectOptions) -> Result<Self, Error> {
        Ok(Database::connect(connection_options).await?)
    }

    fn name() -> &'static str
    where
        Self: Sized,
    {
        "database_sql<DatabaseConnection>"
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
}
#[derive(Debug, Clone)]
pub struct DatabaseDirectory<Connection: DatabaseDirectoryTrait> {
    pub(crate) database: Connection,
}

impl<Connection: DatabaseDirectoryTrait> Service for DatabaseDirectory<Connection> {
    type ServiceConfig = DatabaseConfig;
    type ServiceError = Error;
}

#[async_trait]
impl<Connection: DatabaseDirectoryTrait> Directory for DatabaseDirectory<Connection> {
    fn directory_name() -> &'static str
    where
        Self: Sized,
    {
        Connection::name()
    }

    async fn load(config: Self::ServiceConfig) -> Result<Self, Self::ServiceError>
    where
        Self: Sized,
    {
        // TODO pool options
        let database =
            Connection::connect(ConnectOptions::new(config.database.to_string())).await?;
        Ok(Self { database })
    }

    async fn get_account(&self, username: String) -> Result<Option<Account>, Self::ServiceError> {
        todo!()
    }

    async fn login_account(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<Account>, Self::ServiceError> {
        todo!()
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
