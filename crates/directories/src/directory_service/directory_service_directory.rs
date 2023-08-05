use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::BytesMut;
use futures_core::future::LocalBoxFuture;
use futures_lite::{
    AsyncReadExt as FuturesLiteAsyncRead, AsyncWriteExt as FuturesLightAsyncWriteExt,
};
use interprocess::local_socket::tokio::LocalSocketStream;
use thiserror::Error;
use tracing::trace;

use utils::account::Account;
use utils::interprocess_guard::InterprocessConnectionInner;
use utils::service::{Service, ServiceAccess};
use utils::service_configuration::ServiceConfigurationResponse;

use crate::directory_service::packets::{FromServicePackets, ToServicePackets};
use crate::directory_type::Directory;
use crate::{ValidateDirectoryRequest, SOCKET_NAME};

#[derive(Debug, Error)]
pub enum DirectoryServiceError {
    #[error("Internal Service Error: {0}")]
    Service(String),
    #[error(transparent)]
    Connection(#[from] io::Error),
}
pub struct DirectoryServiceDirectory(Arc<InterprocessConnectionInner>);
impl Deref for DirectoryServiceDirectory {
    type Target = InterprocessConnectionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct DirectoryServiceDirectoryAccess;

impl ServiceAccess for DirectoryServiceDirectoryAccess {
    type ServiceResponse = DirectoryServiceDirectory;
    type Error = DirectoryServiceError;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<DirectoryServiceDirectory, DirectoryServiceError>>
                + Send
                + 'static,
        >,
    >;

    fn get_service(&self) -> Self::Future {
        Box::pin(async move { DirectoryServiceDirectory::load(()).await })
    }
}
impl DirectoryServiceDirectory {
    async fn write_packet(
        connection: &mut LocalSocketStream,
        packet: ToServicePackets,
    ) -> Result<(), DirectoryServiceError> {
        let rkye = rkyv::to_bytes::<_, 256>(&packet).map_err(|e| {
            DirectoryServiceError::Connection(io::Error::new(ErrorKind::InvalidData, e))
        })?;
        trace!("Sending packet: {:?} with size {}", packet, rkye.len());
        trace!("Content: {:?}", rkye);
        connection
            .write_all(&(rkye.len() as u32).to_be_bytes())
            .await
            .map_err(DirectoryServiceError::Connection)?;
        connection
            .write_all(rkye.as_slice())
            .await
            .map_err(DirectoryServiceError::Connection)
    }
    async fn get_packet(
        connection: &mut LocalSocketStream,
    ) -> Result<FromServicePackets, DirectoryServiceError> {
        let mut len = [0u8; 4];
        connection
            .read_exact(&mut len)
            .await
            .map_err(DirectoryServiceError::Connection)?;
        let mut len = u32::from_be_bytes(len);

        let mut account = BytesMut::zeroed(len as usize);
        connection
            .read_exact(account.deref_mut())
            .await
            .map_err(DirectoryServiceError::Connection)?;

        rkyv::from_bytes::<FromServicePackets>(&account).map_err(|e| {
            DirectoryServiceError::Connection(io::Error::new(ErrorKind::InvalidData, e.to_string()))
        })
    }
}
impl Service for DirectoryServiceDirectory {
    type ServiceConfig = ();
    type ServiceError = DirectoryServiceError;
}
#[async_trait]
impl Directory for DirectoryServiceDirectory {
    fn directory_name() -> &'static str
    where
        Self: Sized,
    {
        "Directory Service Connection"
    }

    async fn load(_: Self::ServiceConfig) -> Result<Self, Self::ServiceError>
    where
        Self: Sized,
    {
        let connection = LocalSocketStream::connect(SOCKET_NAME)
            .await
            .expect("Failed to connect to directory service");

        Ok(Self(Arc::new(InterprocessConnectionInner::new(connection))))
    }

    async fn get_account(
        &self,
        username: String,
    ) -> Result<Option<Account>, DirectoryServiceError> {
        let mut connection = self.get_guard_panic();
        Self::write_packet(
            connection.deref_mut(),
            ToServicePackets::GetAccount(username.into()),
        )
        .await?;
        Self::get_packet(connection.deref_mut())
            .await
            .map(|p| match p {
                FromServicePackets::GetAccount(a) => a,
                _ => None,
            })
    }

    async fn login_account(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<Account>, Self::ServiceError> {
        let mut connection = self.get_guard_panic();
        Self::write_packet(
            connection.deref_mut(),
            ToServicePackets::LoginAccount { username, password },
        )
        .await?;
        Self::get_packet(connection.deref_mut())
            .await
            .map(|p| match p {
                FromServicePackets::LoginAccount(a) => a,
                _ => None,
            })
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
