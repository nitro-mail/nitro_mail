use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    pub has_mail_box: bool,
    #[sea_orm(unique, column_type = "Text")]
    pub group_name: String,
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
    GroupEmail,
}

impl Related<super::group_account_rels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupAccountRel.def()
    }
}
impl Related<super::emails::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupEmail.def()
    }
}
