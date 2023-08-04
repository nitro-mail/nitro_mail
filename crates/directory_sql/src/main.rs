use std::io::read_to_string;
use std::path::PathBuf;

use sea_orm::DatabaseConnection;
use thiserror::__private::PathAsDisplay;

use directories::directory_service::DirectoryService;
use directories::directory_type::Directory;
use migration::MigratorTrait;
use utils::Config;

use crate::database_config::DatabaseConfig;
use crate::database_directory::DatabaseDirectory;

pub mod database_config;
pub mod database_directory;
#[cfg(test)]
pub mod database_tests;

#[tokio::main]
async fn main() {
    let database_config = PathBuf::from(DatabaseConfig::config_name());
    let config: DatabaseConfig = if !database_config.exists() {
        let mut file = std::fs::File::create(&database_config).unwrap();
        let config = DatabaseConfig::default();
        config.write(&mut file).unwrap();
        config
    } else {
        let mut file = std::fs::File::open(&database_config).unwrap();
        toml::from_str(read_to_string(&mut file).unwrap().as_str())
            .expect(&format!("Unable to parse {}", database_config.as_display()))
    };
    let directory = DatabaseDirectory::<DatabaseConnection>::load(config)
        .await
        .unwrap();
    let service = DirectoryService::new(directory);
    service.run().await;
}
