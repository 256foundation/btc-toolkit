use crate::errors::{FetchError, FetchResult};
use asic_rs::{data::miner::MinerData, MinerFactory};
use std::net::IpAddr;
use tokio::runtime::Runtime;

/// Fetches complete miner data for a single IP address.
///
/// This function creates a dedicated Tokio runtime and blocks on the async fetch operation.
/// Use this for synchronous contexts or when called from Iced's Task::perform.
///
/// # Errors
///
/// Returns `FetchError` if:
/// - Runtime creation fails
/// - Miner factory creation fails
/// - No miner is found at the IP
/// - Data fetching fails
pub fn fetch_full_miner_data(ip: IpAddr) -> FetchResult<MinerData> {
    // Create dedicated Tokio runtime for this fetch
    let rt = Runtime::new()
        .map_err(|e| FetchError::RuntimeCreation(e.to_string()))?;

    rt.block_on(async move {
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
