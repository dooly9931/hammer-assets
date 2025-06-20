//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

use super::sea_orm_active_enums::DataProvider;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "balance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub wallet_id: i32,
    pub time: TimeDateTimeWithTimeZone,
    pub provider: DataProvider,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::balance_entry::Entity")]
    BalanceEntry,
    #[sea_orm(
        belongs_to = "super::wallet::Entity",
        from = "Column::WalletId",
        to = "super::wallet::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Wallet,
}

impl Related<super::balance_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BalanceEntry.def()
    }
}

impl Related<super::wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wallet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
