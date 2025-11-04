use crate::errors::{FetchError, FetchResult};
use asic_rs::{data::miner::MinerData, MinerFactory};
use std::net::IpAddr;
use tokio::runtime::Handle;

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
    let factory = MinerFactory::new();

    // Get the miner at the specified IP
    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    // Fetch ALL data (not partial like the scanner does)
    Ok(miner.get_data().await)
}
