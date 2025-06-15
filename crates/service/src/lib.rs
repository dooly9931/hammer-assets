mod query;
pub mod types;

use query::QueryService;
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct HammerService {
    pub query: QueryService,
}

impl HammerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            query: QueryService::new(db),
        }
    }

    pub async fn from_url(url: &str) -> Result<Self, sea_orm::DbErr> {
        let db = Database::connect(url).await?;
        Ok(Self::new(db))
    }

    /// Create a new service instance with database connection from environment
    pub async fn from_env() -> Result<Self, sea_orm::DbErr> {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        Self::from_url(&database_url).await
    }
}
