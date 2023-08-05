use std::env::current_dir;
use std::io::read_to_string;
use std::path::PathBuf;

use sea_orm::DatabaseConnection;
use thiserror::__private::PathAsDisplay;

use directories::directory_service::DirectoryService;
use directories::directory_type::Directory;
use migration::MigratorTrait;
use utils::configs::Config;

use crate::database_config::DatabaseConfig;
use crate::database_directory::DatabaseDirectory;

pub mod database_config;
pub mod database_directory;
#[cfg(test)]
pub mod database_tests;

#[tokio::main]
async fn main() {
    let database_config = DatabaseConfig::get_or_save_default(current_dir().unwrap()).unwrap();
    let directory = DatabaseDirectory::<DatabaseConnection>::load(database_config)
        .await
        .unwrap();
    let service = DirectoryService::new(directory);
    service.run().await;
}
