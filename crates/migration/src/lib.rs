pub use sea_orm_migration::prelude::*;

mod m20241201_000001_create_wallet_tables;
mod m20241201_000002_create_currency_tables;
mod m20241201_000003_create_balance_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241201_000001_create_wallet_tables::Migration),
            Box::new(m20241201_000002_create_currency_tables::Migration),
            Box::new(m20241201_000003_create_balance_tables::Migration),
        ]
    }
}
