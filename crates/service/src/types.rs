use hammer_entity::sea_orm_active_enums::{
    AssetScope as EntityAssetScope, DataProvider as EntityDataProvider,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// New wallet data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWallet {
    pub scope: AssetScope,
    pub parent_id: Option<i32>,
}

/// New wallet metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWalletMetadata {
    pub wallet_id: i32,
    pub alias: String,
    pub address: Option<String>,
}

/// New balance data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBalance {
    pub wallet_id: i32,
    pub time: OffsetDateTime,
    pub provider: DataProvider,
}

/// New balance entry data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBalanceEntry {
    pub balance_id: i32,
    pub raw_currency: String,
    pub amount: Decimal,
}

/// New price data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPrice {
    pub currency: String,
    pub time: OffsetDateTime,
    pub value: Decimal,
    pub liquidity: Decimal,
    pub provider: DataProvider,
}

/// New currency data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCurrency {
    pub name: String,
}

/// New currency mapping data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCurrencyMap {
    pub scope: AssetScope,
    pub raw_currency: String,
    pub currency: String,
}

/// New balance priority data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBalancePriority {
    pub wallet_id: i32,
    pub provider: DataProvider,
    pub priority: i32,
}

/// New price priority data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPricePriority {
    pub currency: String,
    pub provider: DataProvider,
    pub priority: i32,
}

/// Asset scope enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetScope {
    Binance,
    Upbit,
    Spot,
    Future,
    Ethereum,
    Pendle2,
    Stakestone,
    Other,
}

impl From<EntityAssetScope> for AssetScope {
    fn from(value: EntityAssetScope) -> Self {
        match value {
            EntityAssetScope::Binance => AssetScope::Binance,
            EntityAssetScope::Upbit => AssetScope::Upbit,
            EntityAssetScope::Spot => AssetScope::Spot,
            EntityAssetScope::Future => AssetScope::Future,
            EntityAssetScope::Ethereum => AssetScope::Ethereum,
            EntityAssetScope::Pendle2 => AssetScope::Pendle2,
            EntityAssetScope::Stakestone => AssetScope::Stakestone,
            EntityAssetScope::Other => AssetScope::Other,
        }
    }
}

impl From<AssetScope> for EntityAssetScope {
    fn from(value: AssetScope) -> Self {
        match value {
            AssetScope::Binance => EntityAssetScope::Binance,
            AssetScope::Upbit => EntityAssetScope::Upbit,
            AssetScope::Spot => EntityAssetScope::Spot,
            AssetScope::Future => EntityAssetScope::Future,
            AssetScope::Ethereum => EntityAssetScope::Ethereum,
            AssetScope::Pendle2 => EntityAssetScope::Pendle2,
            AssetScope::Stakestone => EntityAssetScope::Stakestone,
            AssetScope::Other => EntityAssetScope::Other,
        }
    }
}

/// Data provider enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataProvider {
    Cam,
    Ccxt,
    Debank,
}

impl From<EntityDataProvider> for DataProvider {
    fn from(value: EntityDataProvider) -> Self {
        match value {
            EntityDataProvider::Cam => DataProvider::Cam,
            EntityDataProvider::Ccxt => DataProvider::Ccxt,
            EntityDataProvider::Debank => DataProvider::Debank,
        }
    }
}

impl From<DataProvider> for EntityDataProvider {
    fn from(value: DataProvider) -> Self {
        match value {
            DataProvider::Cam => EntityDataProvider::Cam,
            DataProvider::Ccxt => EntityDataProvider::Ccxt,
            DataProvider::Debank => EntityDataProvider::Debank,
        }
    }
}
