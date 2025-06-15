use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create asset scope enum
        manager
            .create_type(
                Type::create()
                    .as_enum(AssetScope::Table)
                    .values(AssetScope::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // Create data provider enum
        manager
            .create_type(
                Type::create()
                    .as_enum(DataProvider::Table)
                    .values(DataProvider::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // Create wallet table
        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .col(pk_auto(Wallet::Id))
                    .col(integer_null(Wallet::ParentId))
                    .col(enumeration(
                        Wallet::Scope,
                        AssetScope::Table,
                        AssetScope::iter().skip(1),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallet-parent_id")
                            .from(Wallet::Table, Wallet::ParentId)
                            .to(Wallet::Table, Wallet::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create wallet metadata table
        manager
            .create_table(
                Table::create()
                    .table(WalletMetadata::Table)
                    .col(pk_auto(WalletMetadata::Id))
                    .col(integer(WalletMetadata::WalletId))
                    .col(string(WalletMetadata::Alias))
                    .col(string_null(WalletMetadata::Address)) // Nullable for CEFI wallets
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallet_metadata-wallet_id")
                            .from(WalletMetadata::Table, WalletMetadata::WalletId)
                            .to(Wallet::Table, Wallet::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables
        manager
            .drop_table(Table::drop().table(WalletMetadata::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await?;

        // Drop enums
        manager
            .drop_type(Type::drop().name(DataProvider::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AssetScope::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Wallet {
    Table,
    Id,
    ParentId,
    Scope,
}

#[derive(DeriveIden)]
pub enum WalletMetadata {
    Table,
    Address,
    Alias,
    Id,
    WalletId,
}

#[derive(DeriveIden, EnumIter)]
pub enum DataProvider {
    Table,
    Cam,
    Ccxt,
    Debank,
}

#[derive(DeriveIden, EnumIter)]
pub enum AssetScope {
    Table,
    // CEFI scopes
    Binance,
    Upbit,
    Spot,
    Future,
    // DEFI scopes
    Ethereum,
    Pendle2,
    Stakestone,
    // Common
    Other,
} // TODO: should handle alias changes, level distinction, and time-based ownership changes
