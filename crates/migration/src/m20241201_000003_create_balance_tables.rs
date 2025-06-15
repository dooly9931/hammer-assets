use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, schema::*};

// Use existing enums from migration 001
use crate::m20241201_000001_create_wallet_tables::DataProvider;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create balance table
        manager
            .create_table(
                Table::create()
                    .table(Balance::Table)
                    .col(pk_auto(Balance::Id))
                    .col(integer(Balance::WalletId))
                    .col(timestamp_with_time_zone(Balance::Time))
                    .col(enumeration(
                        Balance::Provider,
                        DataProvider::Table,
                        DataProvider::iter().skip(1),
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-balance-wallet_id")
                            .from(Balance::Table, Balance::WalletId)
                            .to(
                                crate::m20241201_000001_create_wallet_tables::Wallet::Table,
                                crate::m20241201_000001_create_wallet_tables::Wallet::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create balance entry table
        manager
            .create_table(
                Table::create()
                    .table(BalanceEntry::Table)
                    .col(pk_auto(BalanceEntry::Id))
                    .col(integer(BalanceEntry::BalanceId))
                    .col(string(BalanceEntry::RawCurrency))
                    .col(decimal_len(BalanceEntry::Amount, 20, 8))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-balance_entry-balance_id")
                            .from(BalanceEntry::Table, BalanceEntry::BalanceId)
                            .to(Balance::Table, Balance::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create balance priority table
        manager
            .create_table(
                Table::create()
                    .table(BalancePriority::Table)
                    .col(pk_auto(BalancePriority::Id))
                    .col(integer(BalancePriority::WalletId))
                    .col(enumeration(
                        BalancePriority::Provider,
                        DataProvider::Table,
                        DataProvider::iter().skip(1),
                    ))
                    .col(integer(BalancePriority::Priority))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-balance_priority-wallet_id")
                            .from(BalancePriority::Table, BalancePriority::WalletId)
                            .to(
                                crate::m20241201_000001_create_wallet_tables::Wallet::Table,
                                crate::m20241201_000001_create_wallet_tables::Wallet::Id,
                            )
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
            .drop_table(Table::drop().table(BalancePriority::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(BalanceEntry::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Balance::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Balance {
    Table,
    Id,
    WalletId,
    Time,
    Provider,
}

#[derive(DeriveIden)]
pub enum BalanceEntry {
    Table,
    Id,
    BalanceId,
    RawCurrency,
    Amount,
}

#[derive(DeriveIden)]
pub enum BalancePriority {
    Table,
    Id,
    WalletId,
    Provider,
    Priority,
}
