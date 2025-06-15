//! Hammer Assets Worker Binary
//!
//! This binary runs the periodic data fetching workers.

use hammer_worker::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run()
}
