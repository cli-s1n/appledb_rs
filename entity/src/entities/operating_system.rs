//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "operating_system")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::operating_system_version::Entity")]
    OperatingSystemVersion,
}

impl Related<super::operating_system_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OperatingSystemVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
