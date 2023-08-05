use sea_orm::prelude::*;
use serde::Serialize;
use std::any::type_name;
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, IntoStaticStr};
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveActiveEnum,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "Text")]
#[strum(serialize_all = "snake_case")]
pub enum SystemConfigurationOptions {
    #[sea_orm(string_value = "group_namespace")]
    GroupNamespace,
    #[sea_orm(string_value = "account_namespace")]
    AccountNamespace,
}
pub type SystemConfigurationOptionWithType = (&'static str, &'static str);
impl SystemConfigurationOptions {
    pub fn options_iter() -> Vec<SystemConfigurationOptionWithType> {
        vec![
            (
                SystemConfigurationOptions::GroupNamespace.as_ref(),
                type_name::<Uuid>(),
            ),
            (
                SystemConfigurationOptions::AccountNamespace.as_ref(),
                type_name::<Uuid>(),
            ),
        ]
    }
}
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "system_configuration")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(unique)]
    pub key: SystemConfigurationOptions,
    pub value: Vec<u8>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
