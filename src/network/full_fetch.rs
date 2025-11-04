use crate::errors::{FetchError, FetchResult};
use asic_rs::{data::miner::MinerData, MinerFactory};
use std::net::IpAddr;
use tokio::runtime::Handle;

/// Fetches complete miner data for a single IP address (blocking version).
///
/// **⚠️ WARNING**: This function will panic if called from within an async runtime context
/// (e.g., from within `Task::perform` or any tokio task). Use `fetch_full_miner_data_async`
/// instead when you're already in an async context.
///
/// This function uses the shared Tokio runtime (via iced's tokio feature) and blocks
/// on the async fetch operation. Only use this from synchronous, non-async contexts.
///
/// # Panics
///
/// Panics if called from within a tokio runtime context.
///
/// # Errors
///
/// Returns `FetchError` if:
/// - No tokio runtime is available (should not happen with iced's tokio feature)
/// - Miner factory creation fails
/// - No miner is found at the IP
/// - Data fetching fails
pub fn fetch_full_miner_data(ip: IpAddr) -> FetchResult<MinerData> {
    // Use the current tokio runtime handle (shared via iced's tokio feature)
    let handle = Handle::try_current()
        .map_err(|e| FetchError::RuntimeCreation(format!("No tokio runtime available: {}", e)))?;

    // Note: This will panic if called from within a runtime context
    // Use fetch_full_miner_data_async instead in async contexts
    handle.block_on(async move {
        fetch_full_miner_data_internal(ip).await
    })
}

/// Async version for use in async contexts.
///
/// # Errors
///
/// Returns `FetchError` if:
/// - Miner factory creation fails
/// - No miner is found at the IP
/// - Data fetching fails
pub async fn fetch_full_miner_data_async(ip: IpAddr) -> FetchResult<MinerData> {
    fetch_full_miner_data_internal(ip).await
}

/// Internal implementation for fetching miner data.
async fn fetch_full_miner_data_internal(ip: IpAddr) -> FetchResult<MinerData> {
    // Create a factory for just this one IP using CIDR /32 notation
    let subnet = format!("{}/32", ip);
    let factory = MinerFactory::new()
        .with_subnet(&subnet)
        .map_err(|e| FetchError::FactoryCreation(format!("{}: {}", subnet, e)))?;

    // Get the miner at the specified IP
    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    // Fetch ALL data (not partial like the scanner does)
    Ok(miner.get_data().await)
}
