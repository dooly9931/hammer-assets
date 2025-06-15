//! Hammer Assets Worker
//!
//! This crate provides periodic data fetching and processing workers for the hammer-assets system.

use std::time::Duration;

use anyhow::Result;
use hammer_service::HammerService;
use sea_orm::{ConnectOptions, Database};
use tokio::time::{MissedTickBehavior, interval};
use tracing::{error, info};

mod balance_worker;
mod price_worker;
mod wallet_worker;

/// Main worker function that spawns all periodic workers
#[tokio::main]
pub async fn run() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Hammer Assets Worker");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Establish database connection
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");

    let connect_options = ConnectOptions::new(database_url)
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(300))
        .to_owned();

    let conn = Database::connect(connect_options).await?;
    info!("Database connection established");

    // Create service instance
    let svc = HammerService::new(conn);

    // Spawn workers
    spawn_balance_worker(svc.clone());
    spawn_price_worker(svc.clone());
    spawn_wallet_worker(svc.clone());

    info!("All workers spawned successfully");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");

    Ok(())
}

/// Spawns a worker that periodically fetches balance data
fn spawn_balance_worker(svc: HammerService) {
    let mut interval = interval(Duration::from_secs(300)); // Every 5 minutes
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            if let Err(e) = balance_worker::fetch_balances(&svc).await {
                error!("Failed to fetch balances: {:#?}", e);
            }
        }
    });
}

/// Spawns a worker that periodically fetches price data
fn spawn_price_worker(svc: HammerService) {
    let mut interval = interval(Duration::from_secs(60)); // Every minute
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            if let Err(e) = price_worker::fetch_prices(&svc).await {
                error!("Failed to fetch prices: {:#?}", e);
            }
        }
    });
}

/// Spawns a worker that periodically syncs wallet data
fn spawn_wallet_worker(svc: HammerService) {
    let mut interval = interval(Duration::from_secs(3600)); // Every hour
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            if let Err(e) = wallet_worker::sync_wallets(&svc).await {
                error!("Failed to sync wallets: {:#?}", e);
            }
        }
    });
}
