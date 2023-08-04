use async_trait::async_trait;

use utils::service::Service;
use utils::Config;

#[async_trait]
pub trait Storage: Service {
    fn storage_name() -> &'static str
    where
        Self: Sized;
    fn storage_path(&self) -> String;
}
