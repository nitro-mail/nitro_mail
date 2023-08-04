use rkyv::Archive;
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString, IntoStaticStr};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    rkyv::Deserialize,
    rkyv::Serialize,
    rkyv::Archive,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
#[archive(compare(PartialEq), check_bytes)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::prelude::DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum AccountType {
    #[default]
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Individual"))]
    Individual,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Default,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    AsRefStr,
    IntoStaticStr,
    EnumIs,
    EnumString,
    Display,
    EnumIter,
)]
#[archive(compare(PartialEq), check_bytes)]
#[cfg_attr(feature = "sea-orm", derive(sea_orm::prelude::DeriveActiveEnum))]
#[cfg_attr(feature = "sea-orm", sea_orm(rs_type = "String", db_type = "Text"))]
pub enum EmailType {
    #[default]
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Primary"))]
    Primary,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "Alias"))]
    Alias,
    #[cfg_attr(feature = "sea-orm", sea_orm(string_value = "List"))]
    List,
}
