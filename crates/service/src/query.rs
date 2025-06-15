use sea_orm::DatabaseConnection;

mod balance;
mod currency;
mod price;
mod wallet;

#[derive(Clone)]
pub struct QueryService {
    db: DatabaseConnection,
}

impl QueryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
