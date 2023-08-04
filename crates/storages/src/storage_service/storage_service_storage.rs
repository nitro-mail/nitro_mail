use std::io;

use futures_lite::AsyncWriteExt as FuturesLightAsyncWriteExt;
use thiserror::Error;
use utils::interprocess_guard::InterprocessConnectionInner;
use utils::service::Service;

use crate::storage_type::Storage;

#[derive(Debug, Error)]
pub enum StorageServiceError {
    #[error("Internal Service Error: {0}")]
    Service(String),
    #[error(transparent)]
    Connection(#[from] io::Error),
}

pub struct StorageService(InterprocessConnectionInner);

impl Service for StorageService {
    type ServiceConfig = ();
    type ServiceError = StorageServiceError;
}

impl Storage for StorageService {
    fn storage_name() -> &'static str
    where
        Self: Sized,
    {
        "storage_service"
    }

    fn storage_path(&self) -> String {
        todo!()
    }
}
