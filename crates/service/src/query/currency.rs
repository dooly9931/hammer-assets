use crate::types::{NewCurrency, NewCurrencyMap};
use hammer_entity::{currency, currency_map};
use sea_orm::{Set, entity::prelude::*};

use super::QueryService;

impl QueryService {
    /// Get all currencies
    pub async fn get_currencies(&self) -> Result<Vec<currency::Model>, DbErr> {
        currency::Entity::find().all(&self.db).await
    }

    /// Get currency by name
    pub async fn get_currency_by_name(&self, name: &str) -> Result<Option<currency::Model>, DbErr> {
        currency::Entity::find()
            .filter(currency::Column::Name.eq(name))
            .one(&self.db)
            .await
    }

    /// Create currency
    pub async fn create_currency(
        &self,
        new_currency: NewCurrency,
    ) -> Result<currency::Model, DbErr> {
        let currency = currency::ActiveModel {
            name: Set(new_currency.name),
        };
        currency.insert(&self.db).await
    }

    /// Delete currency
    pub async fn delete_currency(&self, name: &str) -> Result<bool, DbErr> {
        let result = currency::Entity::delete_many()
            .filter(currency::Column::Name.eq(name))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }

    /// Get currency mappings by scope
    pub async fn get_currency_mappings_by_scope(
        &self,
        scope: &str,
    ) -> Result<Vec<currency_map::Model>, DbErr> {
        currency_map::Entity::find()
            .filter(currency_map::Column::Scope.eq(scope))
            .all(&self.db)
            .await
    }

    /// Get currency mapping by raw currency and scope
    pub async fn get_currency_mapping(
        &self,
        scope: &str,
        raw_currency: &str,
    ) -> Result<Option<currency_map::Model>, DbErr> {
        currency_map::Entity::find()
            .filter(currency_map::Column::Scope.eq(scope))
            .filter(currency_map::Column::RawCurrency.eq(raw_currency))
            .one(&self.db)
            .await
    }

    /// Create currency mapping
    pub async fn create_currency_mapping(
        &self,
        new_mapping: NewCurrencyMap,
    ) -> Result<currency_map::Model, DbErr> {
        let mapping = currency_map::ActiveModel {
            scope: Set(new_mapping.scope.into()),
            raw_currency: Set(new_mapping.raw_currency),
            currency: Set(new_mapping.currency),
            ..Default::default()
        };
        mapping.insert(&self.db).await
    }

    /// Update currency mapping
    pub async fn update_currency_mapping(
        &self,
        id: i32,
        new_mapping: NewCurrencyMap,
    ) -> Result<currency_map::Model, DbErr> {
        let mapping = currency_map::ActiveModel {
            id: Set(id),
            scope: Set(new_mapping.scope.into()),
            raw_currency: Set(new_mapping.raw_currency),
            currency: Set(new_mapping.currency),
        };
        mapping.update(&self.db).await
    }

    /// Delete currency mapping
    pub async fn delete_currency_mapping(&self, id: i32) -> Result<bool, DbErr> {
        let result = currency_map::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected == 1)
    }

    /// Get all currency mappings
    pub async fn get_all_currency_mappings(&self) -> Result<Vec<currency_map::Model>, DbErr> {
        currency_map::Entity::find().all(&self.db).await
    }
}
