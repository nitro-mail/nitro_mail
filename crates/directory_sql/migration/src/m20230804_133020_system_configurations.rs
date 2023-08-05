use crate::sea_orm::{DbBackend, EntityName, Iterable, Statement};
use entities::system_configuration::{Column, Entity, SystemConfigurationOptions};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{DatabaseBackend, Schema};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let key_options = SystemConfigurationOptions::options_iter()
            .into_iter()
            .map(|o| format!("{}: {}", o.0, o.1))
            .collect::<Vec<String>>();
        let values = key_options.join(",");
        let table_comment = format!("Stores system configuration values. Do not modify unless you know what you are doing. Changing a value could corrupt the system. Key Name: Type ({})", values);
        let table = Table::create()
            .table(Entity.table_ref())
            .col(
                ColumnDef::new(Column::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(
                ColumnDef::new(Column::Key)
                    .string()
                    .not_null()
                    .unique_key()
                    // Only applied to mysql
                    .comment(format!(
                        "One of the keys from the SystemConfigurationOptions enum ({}).",
                        values
                    )),
            )
            .col(ColumnDef::new(Column::Value).blob(BlobSize::Tiny))
            .comment(table_comment)
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemConfiguration::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SystemConfiguration {
    Table,
    Id,
    Key,
    Value,
}
