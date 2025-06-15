use sea_orm::Iterable;
use sea_orm_migration::{prelude::*, schema::*};

// Use existing enums from migration 001
use crate::m20241201_000001_create_wallet_tables::{AssetScope, DataProvider};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create currency table
        manager
            .create_table(
                Table::create()
                    .table(Currency::Table)
                    .col(ColumnDef::new(Currency::Name).string().primary_key())
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create currency map table
        manager
            .create_table(
                Table::create()
                    .table(CurrencyMap::Table)
                    .col(pk_auto(CurrencyMap::Id))
                    .col(enumeration(
                        CurrencyMap::Scope,
                        AssetScope::Table,
                        AssetScope::iter().skip(1),
                    ))
                    .col(string(CurrencyMap::RawCurrency))
                    .col(string(CurrencyMap::Currency))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-currency_map-currency")
                            .from(CurrencyMap::Table, CurrencyMap::Currency)
                            .to(Currency::Table, Currency::Name)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create price table
        manager
            .create_table(
                Table::create()
                    .table(Price::Table)
                    .col(pk_auto(Price::Id))
                    .col(string(Price::Currency))
                    .col(timestamp_with_time_zone(Price::Time))
                    .col(decimal_len(Price::Value, 20, 8))
                    .col(decimal_len(Price::Liquidity, 20, 8))
                    .col(enumeration(
                        Price::Provider,
                        DataProvider::Table,
                        DataProvider::iter().skip(1),
                    ))
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create price provider table
        manager
            .create_table(
                Table::create()
                    .table(PriceProvider::Table)
                    .col(pk_auto(PriceProvider::Id))
                    .col(string(PriceProvider::Currency))
                    .col(enumeration(
                        PriceProvider::Provider,
                        DataProvider::Table,
                        DataProvider::iter().skip(1),
                    ))
                    .col(integer(PriceProvider::Priority))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-price_provider-currency")
                            .from(PriceProvider::Table, PriceProvider::Currency)
                            .to(Currency::Table, Currency::Name)
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
            .drop_table(Table::drop().table(PriceProvider::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Price::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CurrencyMap::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Currency::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Currency {
    Table,
    Name,
}

#[derive(DeriveIden)]
pub enum CurrencyMap {
    Table,
    Id,
    Scope,
    RawCurrency,
    Currency,
}

#[derive(DeriveIden)]
pub enum Price {
    Table,
    Id,
    Currency,
    Liquidity,
    Time,
    Value,
    Provider,
}

#[derive(DeriveIden)]
pub enum PriceProvider {
    Table,
    Id,
    Currency,
    Provider,
    Priority,
}
