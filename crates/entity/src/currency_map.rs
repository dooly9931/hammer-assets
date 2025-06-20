//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use super::sea_orm_active_enums::AssetScope;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "currency_map")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub scope: AssetScope,
    pub raw_currency: String,
    pub currency: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::Currency",
        to = "super::currency::Column::Name",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Currency,
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
