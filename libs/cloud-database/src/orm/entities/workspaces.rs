//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "workspaces")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: String,
    pub public: bool,
    pub r#type: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::permission::Entity")]
    Permission,
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
