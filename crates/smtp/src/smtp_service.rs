use crate::smtp_config::SMTPConfig;
use crate::smtp_listener::Instance;
use directories::directory_type::Directory;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use storages::storage_type::Storage;
use thiserror::Error;
use tokio::task::JoinHandle;
use utils::configs::dkim::DKIMConfig;
use utils::configs::domain_configs::DomainConfiguration;
use utils::configs::{Config, IOOrToml};
use utils::service::ServiceAccess;

pub type Configs = (SMTPConfig, DomainConfiguration, DKIMConfig);
#[derive(Debug, Error)]
pub enum SMTPServiceError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Config(#[from] IOOrToml),
    #[error(transparent)]
    GettingDirectoryAccess(Box<dyn Error + Send + Sync + 'static>),
}
pub struct SMTPServiceInner<
    D: Directory,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    S: Storage,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
> {
    pub config: SMTPConfig,
    pub domain_config: DomainConfiguration,
    pub dkim_config: DKIMConfig,
    pub running: AtomicBool,
    pub directory_service_access: DirectoryAccess,
    pub storage_service_access: StorageAccess,
}

impl<
        D: Directory,
        DirectoryAccess: ServiceAccess<ServiceResponse = D>,
        S: Storage,
        StorageAccess: ServiceAccess<ServiceResponse = S>,
    > SMTPService<D, DirectoryAccess, S, StorageAccess>
{
    pub fn start(
        working_directory: PathBuf,
        directory_service_access: DirectoryAccess,
        storage_service_access: StorageAccess,
    ) -> Result<SMTPService<D, DirectoryAccess, S, StorageAccess>, SMTPServiceError> {
        let (smtp_config, domain_config, dkim_config) =
            Configs::get_or_save_default(working_directory)?;

        let service = Arc::new(SMTPServiceInner {
            config: smtp_config.clone(),
            domain_config,
            dkim_config,
            running: AtomicBool::new(true),
            directory_service_access: directory_service_access.clone(),
            storage_service_access: storage_service_access.clone(),
        });
        let mut instances = Vec::with_capacity(smtp_config.hosts.len());
        for host in smtp_config.hosts {
            let instance = Instance {
                service: service.clone(),
                host,
            };
            let handle = tokio::spawn(async move {
                if let Err(e) = instance.run().await {
                    eprintln!("Error in SMTP instance: {:?}", e);
                }
            });
            instances.push(handle);
        }

        Ok(SMTPService {
            inner: service,
            instances,
        })
    }
}
pub struct SMTPService<
    D: Directory,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    S: Storage,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
> {
    pub inner: SMTPServiceAccess<D, DirectoryAccess, S, StorageAccess>,
    instances: Vec<JoinHandle<()>>,
}
pub type SMTPServiceAccess<
    D: Directory,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    S: Storage,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
> = Arc<SMTPServiceInner<D, DirectoryAccess, S, StorageAccess>>;
