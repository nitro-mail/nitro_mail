use async_trait::async_trait;
use futures::future::Ready;
use sea_orm::prelude::*;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectOptions, Database, DatabaseConnection, QueryFilter,
};
use sqlx::Connection;
use std::convert::Infallible;
use std::fmt::Debug;
use thiserror::Error;
use tracing::error;

use directories::directory_type::Directory;
use directories::ValidateDirectoryRequest;
use entities::system_configuration::SystemConfigurationOptions;
use utils::account::Account;
use utils::service::{Service, ServiceAccess};
use utils::service_configuration::{GitInfo, ServiceConfigurationResponse, ServiceType};

use crate::database_config::DatabaseConfig;

#[async_trait]
pub trait DatabaseDirectoryTrait: Send + Sync + 'static + ConnectionTrait + Clone {
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
    DatabaseError(#[from] DbErr),
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
}
#[derive(Debug, Clone)]
pub struct DatabaseDirectory<Connection: DatabaseDirectoryTrait> {
    pub(crate) database: Connection,
}
impl<Connection: DatabaseDirectoryTrait> ServiceAccess for DatabaseDirectory<Connection> {
    type ServiceResponse = Self;
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn get_service(&self) -> Self::Future {
        futures::future::ready(Ok(self.clone()))
    }
}
impl<Connection: DatabaseDirectoryTrait> DatabaseDirectory<Connection> {
    fn log_failed_configuration_check(
        validate_config_request: &ValidateDirectoryRequest,
        user_namespace: &impl Debug,
        group_namespace: &impl Debug,
    ) -> ServiceConfigurationResponse {
        error!("Namespace mismatch");
        error!("User namespace: {:?}", user_namespace);
        error!("Group namespace: {:?}", group_namespace);
        error!("Expected: {:?}", validate_config_request);
        // TODO add instructions
        error!("Please follow the instructions here: https://docs.nitro-mail.kingtux.dev/");
        ServiceConfigurationResponse::NamespaceMismatch {}
    }
    fn get_success_response(new_install: bool) -> ServiceConfigurationResponse {
        ServiceConfigurationResponse::Success {
            new_install,
            internal_service_name: Self::directory_name().to_string(),
            git: GitInfo {
                commit: env!("VERGEN_GIT_SHA").to_string(),
                branch: env!("VERGEN_GIT_BRANCH").to_string(),
                commit_date: env!("VERGEN_GIT_COMMIT_DATE").to_string(),
            },
            service_type: ServiceType::Directory,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
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
        use entities::system_configuration::ActiveModel as SystemConfigurationActiveModel;
        use entities::system_configuration::Column as SystemConfigurationColumn;
        use entities::system_configuration::Entity as SystemConfigurationEntity;
        let user_namespace = SystemConfigurationEntity::find()
            .filter(SystemConfigurationColumn::Key.eq(SystemConfigurationOptions::AccountNamespace))
            .one(&self.database)
            .await?;
        let group_namespace = SystemConfigurationEntity::find()
            .filter(SystemConfigurationColumn::Key.eq(SystemConfigurationOptions::GroupNamespace))
            .one(&self.database)
            .await?;

        // Check if neither of the namespaces are set
        return if user_namespace.is_none() && group_namespace.is_none() {
            // New Install
            SystemConfigurationEntity::insert(SystemConfigurationActiveModel {
                id: Default::default(),
                key: ActiveValue::Set(SystemConfigurationOptions::AccountNamespace),
                value: ActiveValue::Set(
                    validate_config_request
                        .account_namespace
                        .as_bytes()
                        .to_vec(),
                ),
            })
            .exec(&self.database)
            .await?;
            SystemConfigurationEntity::insert(SystemConfigurationActiveModel {
                id: Default::default(),
                key: ActiveValue::Set(SystemConfigurationOptions::GroupNamespace),
                value: ActiveValue::Set(
                    validate_config_request.group_namespace.as_bytes().to_vec(),
                ),
            })
            .exec(&self.database)
            .await?;
            Ok(Self::get_success_response(true))
        } else if let (Some(user_namespace), Some(group_namespace)) =
            (&user_namespace, &group_namespace)
        {
            if user_namespace.value == validate_config_request.account_namespace.as_bytes()
                && group_namespace.value == validate_config_request.group_namespace.as_bytes()
            {
                Ok(Self::get_success_response(false))
            } else {
                Ok(Self::log_failed_configuration_check(
                    &validate_config_request,
                    &user_namespace,
                    &group_namespace,
                ))
            }
        } else {
            Ok(Self::log_failed_configuration_check(
                &validate_config_request,
                &user_namespace,
                &group_namespace,
            ))
        };
    }
}
