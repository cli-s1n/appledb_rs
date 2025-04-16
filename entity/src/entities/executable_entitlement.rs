//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "executable_entitlement")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub executable_operating_system_version_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub entitlement_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::entitlement::Entity",
        from = "Column::EntitlementId",
        to = "super::entitlement::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Entitlement,
    #[sea_orm(
        belongs_to = "super::executable_operating_system_version::Entity",
        from = "Column::ExecutableOperatingSystemVersionId",
        to = "super::executable_operating_system_version::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ExecutableOperatingSystemVersion,
}

impl Related<super::entitlement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entitlement.def()
    }
}

impl Related<super::executable_operating_system_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExecutableOperatingSystemVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
