use crate::smtp_config::SMTPHost;
use crate::smtp_service::{SMTPServiceAccess, SMTPServiceError};
use directories::directory_type::Directory;
use std::net::SocketAddr;

use storages::storage_type::Storage;
use tokio::net::{TcpListener, TcpStream};
use utils::service::ServiceAccess;

pub struct Instance<
    D: Directory,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    S: Storage,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
> {
    pub service: SMTPServiceAccess<D, DirectoryAccess, S, StorageAccess>,
    pub host: SMTPHost,
}
impl<
        D: Directory,
        DirectoryAccess: ServiceAccess<ServiceResponse = D>,
        S: Storage,
        StorageAccess: ServiceAccess<ServiceResponse = S>,
    > Instance<D, DirectoryAccess, S, StorageAccess>
{
    pub async fn run(self) -> Result<(), SMTPServiceError> {
        let socket = TcpListener::bind(self.host.bind).await?;

        while let Ok((stream, addr)) = socket.accept().await {
            let connection = Connection {
                stream,
                addr,

                service: self.service.clone(),
            };
            tokio::spawn(async move {
                if let Err(e) = connection.run().await {
                    eprintln!("Error in SMTP connection: {:?}", e);
                }
            });
        }
        Ok(())
    }
}
