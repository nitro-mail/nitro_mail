use std::env;
use std::path::{Path, PathBuf};

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use migration::{Migrator, MigratorTrait};

async fn connect_to_database() -> Option<DatabaseConnection> {
    println!("Dotenv: {:?}", dotenv::dotenv());
    let url = env::var("DATABASE_URL").ok().unwrap_or_else(|| {
        env::var("POSTGRES_INSTANCE")
            .ok()
            .map(|v| format!("{}nitro_mail_test", v))
            .unwrap_or_else(|| format!("postgres://postgres:postgres@localhost/nitro_mail_test"))
    });
    println!("url: {}", url);
    return match Database::connect(ConnectOptions::new(url)).await {
        Ok(ok) => Some(ok),
        Err(err) => {
            println!("Error connecting to database: {:?}", err);
            None
        }
    };
}
#[tokio::test]
async fn test_database_create() {
    let Some(database_connection) = connect_to_database().await else{
        println!("Unable to connect to database");
        return;
    };
    println!("database_connection: {:?}", database_connection);
    Migrator::fresh(&database_connection)
        .await
        .expect("Failed to run migrations");
}
