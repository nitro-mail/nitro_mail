use std::env::current_dir;
use std::fs::OpenOptions;
use std::path::PathBuf;

use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use directories::directory_service::DirectoryService;
use directories::directory_type::Directory;
use utils::configs::Config;

use crate::test_directory::{TestConfig, TestDirectory};

pub mod test_directory;

#[tokio::main]
async fn main() {
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();
    collector.init();
    let config =
        TestConfig::get_or_save_default(current_dir().expect("Unable to get Working Directory"))
            .unwrap();
    let directory = TestDirectory::load(config).await.unwrap();
    let directory_service = DirectoryService::new(directory);

    directory_service.run().await;
}
