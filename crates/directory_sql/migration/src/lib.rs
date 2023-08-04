pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

macro_rules! entities {
    ($schema:ident,$manager:ident, $($entity_type:path),*) => {
        $(
        {
            let statement = $schema.create_table_from_entity($entity_type);
            $manager.create_table(statement).await?;
        }
        )*
    };
}
pub(crate) use entities;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
