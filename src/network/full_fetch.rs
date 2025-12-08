use crate::errors::{FetchError, FetchResult};
use asic_rs::{MinerFactory, data::miner::MinerData};
use std::net::IpAddr;

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

/// Pause mining on the specified miner.
pub async fn pause_mining_async(ip: IpAddr) -> FetchResult<bool> {
    let factory = MinerFactory::new();

    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    miner
        .pause(None)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))
}

/// Resume mining on the specified miner.
pub async fn resume_mining_async(ip: IpAddr) -> FetchResult<bool> {
    let factory = MinerFactory::new();

    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    miner
        .resume(None)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))
}

/// Toggle the fault light on the specified miner.
pub async fn toggle_fault_light_async(ip: IpAddr) -> FetchResult<bool> {
    let factory = MinerFactory::new();

    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    // Get current state
    let data = miner.get_data().await;
    let current_state = data.light_flashing.unwrap_or(false);
    let new_state = !current_state;

    miner
        .set_fault_light(new_state)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?;

    Ok(new_state)
}

/// Restart the specified miner.
pub async fn restart_miner_async(ip: IpAddr) -> FetchResult<bool> {
    let factory = MinerFactory::new();

    let miner = factory
        .get_miner(ip)
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))?
        .ok_or_else(|| FetchError::MinerNotFound(ip.to_string()))?;

    miner
        .restart()
        .await
        .map_err(|e| FetchError::MinerDataError(e.to_string()))
}
