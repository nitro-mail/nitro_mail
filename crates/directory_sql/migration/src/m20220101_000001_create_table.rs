use sea_orm_migration::prelude::*;

use crate::sea_orm::Schema;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        crate::entities!(schema, manager, entities::GroupEntity);
        crate::entities!(schema, manager, entities::AccountEntity);
        crate::entities!(schema, manager, entities::EmailEntity);
        crate::entities!(schema, manager, entities::GroupAccountRelEntity);
        crate::entities!(schema, manager, entities::SystemConfigurationEntity);

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
