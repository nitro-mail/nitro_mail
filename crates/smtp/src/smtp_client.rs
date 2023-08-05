use crate::smtp_config::SMTPHost;
use crate::smtp_service::{SMTPServiceAccess, SMTPServiceError};
use directories::directory_type::Directory;
use std::net::SocketAddr;

use storages::storage_type::Storage;
use tokio::net::{TcpListener, TcpStream};
use utils::service::ServiceAccess;
pub struct Connection<
    D: Directory,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    S: Storage,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
> {
    pub stream: TcpStream,
    pub addr: SocketAddr,
    pub service: SMTPServiceAccess<D, DirectoryAccess, S, StorageAccess>,
}
impl<
        D: Directory,
        DirectoryAccess: ServiceAccess<ServiceResponse = D>,
        S: Storage,
        StorageAccess: ServiceAccess<ServiceResponse = S>,
    > Connection<D, DirectoryAccess, S, StorageAccess>
{
    pub async fn run(self) -> Result<(), SMTPServiceError> {
        let directory = self
            .service
            .directory_service_access
            .get_service()
            .await
            .map_err(|e| SMTPServiceError::GettingDirectoryAccess(Box::new(e)))?;

        Ok(())
    }
}
