use std::env;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

async fn connect_to_database() -> Option<DatabaseConnection> {
    dotenv::dotenv().ok();
    Database::connect(ConnectOptions::new(
        env::var("DATABASE_URL").unwrap_or_else(|e| {
            "postgres://postgres:password@localhost:5432/test_database".to_string()
        }),
    ))
    .await
    .ok()
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
