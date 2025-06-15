use crate::types::{NewBalance, NewBalanceEntry, NewBalancePriority};
use hammer_entity::{balance, balance_entry, balance_priority};
use sea_orm::{Order, QueryOrder, Set, TransactionTrait, entity::prelude::*};

use super::QueryService;

impl QueryService {
    /// Get balance by ID
    pub async fn get_balance_by_id(&self, id: i32) -> Result<Option<balance::Model>, DbErr> {
        balance::Entity::find_by_id(id).one(&self.db).await
    }

    /// Get balances by wallet ID
    pub async fn get_balances_by_wallet_id(
        &self,
        wallet_id: i32,
    ) -> Result<Vec<balance::Model>, DbErr> {
        balance::Entity::find()
            .filter(balance::Column::WalletId.eq(wallet_id))
            .all(&self.db)
            .await
    }

    /// Get balance with entries
    pub async fn get_balance_with_entries(
        &self,
        balance_id: i32,
    ) -> Result<Option<(balance::Model, Vec<balance_entry::Model>)>, DbErr> {
        balance::Entity::find_by_id(balance_id)
            .find_with_related(balance_entry::Entity)
            .all(&self.db)
            .await
            .map(|results| results.into_iter().next())
    }

    /// Create balance with entries
    pub async fn create_balance_with_entries(
        &self,
        new_balance: NewBalance,
        entries: Vec<NewBalanceEntry>,
    ) -> Result<balance::Model, DbErr> {
        let tx = self.db.begin().await?;

        // Create balance
        let balance = balance::ActiveModel {
            wallet_id: Set(new_balance.wallet_id),
            time: Set(new_balance.time),
            provider: Set(new_balance.provider.into()),
            ..Default::default()
        };

        let balance = match balance.insert(&tx).await {
            Ok(balance) => balance,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        // Create balance entries
        if !entries.is_empty() {
            let balance_entries = entries
                .into_iter()
                .map(|entry| balance_entry::ActiveModel {
                    balance_id: Set(balance.id),
                    raw_currency: Set(entry.raw_currency),
                    amount: Set(entry.amount),
                    ..Default::default()
                })
                .collect::<Vec<_>>();

            if let Err(e) = balance_entry::Entity::insert_many(balance_entries)
                .exec_without_returning(&tx)
                .await
            {
                tx.rollback().await?;
                return Err(e);
            }
        }

        tx.commit().await?;
        Ok(balance)
    }

    /// Update balance
    pub async fn update_balance(
        &self,
        id: i32,
        new_balance: NewBalance,
    ) -> Result<balance::Model, DbErr> {
        let balance = balance::ActiveModel {
            id: Set(id),
            wallet_id: Set(new_balance.wallet_id),
            time: Set(new_balance.time),
            provider: Set(new_balance.provider.into()),
        };
        balance.update(&self.db).await
    }

    /// Delete balance
    pub async fn delete_balance(&self, id: i32) -> Result<bool, DbErr> {
        let result = balance::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected == 1)
    }

    /// Get balance entries by balance ID
    pub async fn get_balance_entries(
        &self,
        balance_id: i32,
    ) -> Result<Vec<balance_entry::Model>, DbErr> {
        balance_entry::Entity::find()
            .filter(balance_entry::Column::BalanceId.eq(balance_id))
            .all(&self.db)
            .await
    }

    /// Add balance entry
    pub async fn add_balance_entry(
        &self,
        new_entry: NewBalanceEntry,
    ) -> Result<balance_entry::Model, DbErr> {
        let entry = balance_entry::ActiveModel {
            balance_id: Set(new_entry.balance_id),
            raw_currency: Set(new_entry.raw_currency),
            amount: Set(new_entry.amount),
            ..Default::default()
        };
        entry.insert(&self.db).await
    }

    /// Update balance entry
    pub async fn update_balance_entry(
        &self,
        id: i32,
        new_entry: NewBalanceEntry,
    ) -> Result<balance_entry::Model, DbErr> {
        let entry = balance_entry::ActiveModel {
            id: Set(id),
            balance_id: Set(new_entry.balance_id),
            raw_currency: Set(new_entry.raw_currency),
            amount: Set(new_entry.amount),
        };
        entry.update(&self.db).await
    }

    /// Delete balance entry
    pub async fn delete_balance_entry(&self, id: i32) -> Result<bool, DbErr> {
        let result = balance_entry::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }

    /// Get balance priorities by wallet ID
    pub async fn get_balance_priorities(
        &self,
        wallet_id: i32,
    ) -> Result<Vec<balance_priority::Model>, DbErr> {
        balance_priority::Entity::find()
            .filter(balance_priority::Column::WalletId.eq(wallet_id))
            .order_by(balance_priority::Column::Priority, Order::Asc)
            .all(&self.db)
            .await
    }

    /// Set balance priority
    pub async fn set_balance_priority(
        &self,
        new_priority: NewBalancePriority,
    ) -> Result<balance_priority::Model, DbErr> {
        let priority = balance_priority::ActiveModel {
            wallet_id: Set(new_priority.wallet_id),
            provider: Set(new_priority.provider.into()),
            priority: Set(new_priority.priority),
            ..Default::default()
        };
        priority.insert(&self.db).await
    }

    /// Update balance priority
    pub async fn update_balance_priority(
        &self,
        id: i32,
        new_priority: NewBalancePriority,
    ) -> Result<balance_priority::Model, DbErr> {
        let priority = balance_priority::ActiveModel {
            id: Set(id),
            wallet_id: Set(new_priority.wallet_id),
            provider: Set(new_priority.provider.into()),
            priority: Set(new_priority.priority),
        };
        priority.update(&self.db).await
    }

    /// Delete balance priority
    pub async fn delete_balance_priority(&self, id: i32) -> Result<bool, DbErr> {
        let result = balance_priority::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }
}
