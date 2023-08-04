use std::fs::OpenOptions;
use std::path::PathBuf;

use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use directories::directory_service::DirectoryService;
use directories::directory_type::Directory;
use utils::Config;

use crate::test_directory::{TestConfig, TestDirectory};

pub mod test_directory;

#[tokio::main]
async fn main() {
    let config_file = PathBuf::from("test-directory-config.toml");
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();
    collector.init();
    let config: TestConfig = if !config_file.exists() {
        let config = TestConfig::default();
        config
            .write(
                &mut OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(config_file)
                    .unwrap(),
            )
            .unwrap();
        config
    } else {
        toml::from_str(&std::fs::read_to_string(config_file).unwrap()).unwrap()
    };
    let directory = TestDirectory::load(config).await.unwrap();
    let directory_service = DirectoryService::new(directory);

    directory_service.run().await;
}
