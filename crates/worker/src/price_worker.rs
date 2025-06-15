//! Price worker for fetching price data

use anyhow::Result;
use hammer_service::HammerService;
use tracing::{info, instrument};

/// Fetches price data from CAM and stores it in the database
#[instrument(skip(_svc))]
pub async fn fetch_prices(_svc: &HammerService) -> Result<()> {
    info!("Starting price fetch");

    // TODO: Implement price fetching logic
    // 1. Get list of currencies from database
    // 2. Fetch prices from CAM for each currency
    // 3. Store prices in database

    info!("Price fetch completed");
    Ok(())
}
