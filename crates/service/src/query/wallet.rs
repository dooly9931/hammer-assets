use crate::types::{NewWallet, NewWalletMetadata};
use hammer_entity::{wallet, wallet_metadata};
use sea_orm::{Set, entity::prelude::*};

use super::QueryService;

impl QueryService {
    /// Get all wallets with their metadata
    pub async fn get_wallets_with_metadata(
        &self,
    ) -> Result<Vec<(wallet::Model, Vec<wallet_metadata::Model>)>, DbErr> {
        wallet::Entity::find()
            .find_with_related(wallet_metadata::Entity)
            .all(&self.db)
            .await
    }

    /// Get wallet by ID
    pub async fn get_wallet_by_id(&self, id: i32) -> Result<Option<wallet::Model>, DbErr> {
        wallet::Entity::find_by_id(id).one(&self.db).await
    }

    /// Create a new wallet
    pub async fn create_wallet(&self, new_wallet: NewWallet) -> Result<wallet::Model, DbErr> {
        let wallet = wallet::ActiveModel {
            scope: Set(new_wallet.scope.into()),
            parent_id: Set(new_wallet.parent_id),
            ..Default::default()
        };
        wallet.insert(&self.db).await
    }

    /// Update wallet
    pub async fn update_wallet(
        &self,
        id: i32,
        new_wallet: NewWallet,
    ) -> Result<wallet::Model, DbErr> {
        let wallet = wallet::ActiveModel {
            id: Set(id),
            scope: Set(new_wallet.scope.into()),
            parent_id: Set(new_wallet.parent_id),
        };
        wallet.update(&self.db).await
    }

    /// Delete wallet
    pub async fn delete_wallet(&self, id: i32) -> Result<bool, DbErr> {
        let result = wallet::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected == 1)
    }

    /// Get wallet metadata by wallet ID
    pub async fn get_wallet_metadata(
        &self,
        wallet_id: i32,
    ) -> Result<Option<wallet_metadata::Model>, DbErr> {
        wallet_metadata::Entity::find()
            .filter(wallet_metadata::Column::WalletId.eq(wallet_id))
            .one(&self.db)
            .await
    }

    /// Create wallet metadata
    pub async fn create_wallet_metadata(
        &self,
        new_metadata: NewWalletMetadata,
    ) -> Result<wallet_metadata::Model, DbErr> {
        let metadata = wallet_metadata::ActiveModel {
            wallet_id: Set(new_metadata.wallet_id),
            alias: Set(new_metadata.alias),
            address: Set(new_metadata.address),
            ..Default::default()
        };
        metadata.insert(&self.db).await
    }

    /// Update wallet metadata
    pub async fn update_wallet_metadata(
        &self,
        id: i32,
        new_metadata: NewWalletMetadata,
    ) -> Result<wallet_metadata::Model, DbErr> {
        let metadata = wallet_metadata::ActiveModel {
            id: Set(id),
            wallet_id: Set(new_metadata.wallet_id),
            alias: Set(new_metadata.alias),
            address: Set(new_metadata.address),
        };
        metadata.update(&self.db).await
    }

    /// Delete wallet metadata
    pub async fn delete_wallet_metadata(&self, id: i32) -> Result<bool, DbErr> {
        let result = wallet_metadata::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }
}
