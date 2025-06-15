use crate::types::{NewPrice, NewPricePriority};
use hammer_entity::{price, price_provider};
use sea_orm::{Order, QueryOrder, Set, entity::prelude::*};

use super::QueryService;

impl QueryService {
    /// Get price by ID
    pub async fn get_price_by_id(&self, id: i32) -> Result<Option<price::Model>, DbErr> {
        price::Entity::find_by_id(id).one(&self.db).await
    }

    /// Get prices by currency
    pub async fn get_prices_by_currency(&self, currency: &str) -> Result<Vec<price::Model>, DbErr> {
        price::Entity::find()
            .filter(price::Column::Currency.eq(currency))
            .order_by(price::Column::Time, Order::Desc)
            .all(&self.db)
            .await
    }

    /// Get latest price by currency
    pub async fn get_latest_price(&self, currency: &str) -> Result<Option<price::Model>, DbErr> {
        price::Entity::find()
            .filter(price::Column::Currency.eq(currency))
            .order_by(price::Column::Time, Order::Desc)
            .one(&self.db)
            .await
    }

    /// Get prices by currency and time range
    pub async fn get_prices_by_currency_and_time_range(
        &self,
        currency: &str,
        start_time: time::OffsetDateTime,
        end_time: time::OffsetDateTime,
    ) -> Result<Vec<price::Model>, DbErr> {
        price::Entity::find()
            .filter(price::Column::Currency.eq(currency))
            .filter(price::Column::Time.gte(start_time))
            .filter(price::Column::Time.lte(end_time))
            .order_by(price::Column::Time, Order::Asc)
            .all(&self.db)
            .await
    }

    /// Create price
    pub async fn create_price(&self, new_price: NewPrice) -> Result<price::Model, DbErr> {
        let price = price::ActiveModel {
            currency: Set(new_price.currency),
            time: Set(new_price.time),
            value: Set(new_price.value),
            liquidity: Set(new_price.liquidity),
            provider: Set(new_price.provider.into()),
            ..Default::default()
        };
        price.insert(&self.db).await
    }

    /// Update price
    pub async fn update_price(&self, id: i32, new_price: NewPrice) -> Result<price::Model, DbErr> {
        let price = price::ActiveModel {
            id: Set(id),
            currency: Set(new_price.currency),
            time: Set(new_price.time),
            value: Set(new_price.value),
            liquidity: Set(new_price.liquidity),
            provider: Set(new_price.provider.into()),
        };
        price.update(&self.db).await
    }

    /// Delete price
    pub async fn delete_price(&self, id: i32) -> Result<bool, DbErr> {
        let result = price::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected == 1)
    }

    /// Get price providers by currency
    pub async fn get_price_providers(
        &self,
        currency: &str,
    ) -> Result<Vec<price_provider::Model>, DbErr> {
        price_provider::Entity::find()
            .filter(price_provider::Column::Currency.eq(currency))
            .order_by(price_provider::Column::Priority, Order::Asc)
            .all(&self.db)
            .await
    }

    /// Set price provider
    pub async fn set_price_provider(
        &self,
        new_priority: NewPricePriority,
    ) -> Result<price_provider::Model, DbErr> {
        let provider = price_provider::ActiveModel {
            currency: Set(new_priority.currency),
            provider: Set(new_priority.provider.into()),
            priority: Set(new_priority.priority),
            ..Default::default()
        };
        provider.insert(&self.db).await
    }

    /// Update price provider
    pub async fn update_price_provider(
        &self,
        id: i32,
        new_priority: NewPricePriority,
    ) -> Result<price_provider::Model, DbErr> {
        let provider = price_provider::ActiveModel {
            id: Set(id),
            currency: Set(new_priority.currency),
            provider: Set(new_priority.provider.into()),
            priority: Set(new_priority.priority),
        };
        provider.update(&self.db).await
    }

    /// Delete price provider
    pub async fn delete_price_provider(&self, id: i32) -> Result<bool, DbErr> {
        let result = price_provider::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }

    /// Get all price providers
    pub async fn get_all_price_providers(&self) -> Result<Vec<price_provider::Model>, DbErr> {
        price_provider::Entity::find()
            .order_by(price_provider::Column::Currency, Order::Asc)
            .order_by(price_provider::Column::Priority, Order::Asc)
            .all(&self.db)
            .await
    }
}
