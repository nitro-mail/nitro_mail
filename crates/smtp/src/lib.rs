use crate::smtp_service::SMTPService;
use directories::directory_type::Directory;
use std::path::PathBuf;
use storages::storage_type::Storage;
use utils::service::ServiceAccess;

pub mod smtp_client;
pub mod smtp_config;
pub mod smtp_listener;
pub mod smtp_service;

pub fn start_smtp_service<
    D: Directory,
    S: Storage,
    DirectoryAccess: ServiceAccess<ServiceResponse = D>,
    StorageAccess: ServiceAccess<ServiceResponse = S>,
>(
    working_directory: PathBuf,
    directory_service_access: DirectoryAccess,
    storage_service_access: StorageAccess,
) -> Result<SMTPService<D, DirectoryAccess, S, StorageAccess>, smtp_service::SMTPServiceError> {
    SMTPService::start(
        working_directory,
        directory_service_access,
        storage_service_access,
    )
}
