use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use utils::common_types::AccountType;
use utils::helper_types::Password;

pub mod database_helpers;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(unique, column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    #[sea_orm(column_type = "Text")]
    pub password: Password,
    #[sea_orm(default_value = "0")]
    pub quota: i64,
    #[sea_orm(default_value = "individual", column_type = "Text")]
    pub account_type: AccountType,
    #[sea_orm(default_value = "true")]
    pub active: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}

// Foreign Key group_id to Group::id

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group_account_rels::Entity")]
    GroupAccountRel,
    #[sea_orm(has_many = "super::emails::Entity")]
    Email,
}

impl Related<super::emails::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Email.def()
    }
}
impl Related<super::group_account_rels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupAccountRel.def()
    }
}
