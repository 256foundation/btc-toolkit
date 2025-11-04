pub mod scanner;
pub mod full_fetch;

use crate::errors::ScannerError;
use asic_rs::miners::factory::MinerFactory;
use scanner::ScanConfig;

/// Validates and creates a MinerFactory from a network range string.
///
/// Supports two formats:
/// - CIDR notation: "192.168.1.0/24"
/// - IP range: "192.168.1.1-100"
///
/// # Errors
///
/// Returns `ScannerError::NetworkRangeInvalid` if the format is invalid
pub fn create_miner_factory(network_range: &str) -> Result<MinerFactory, ScannerError> {
    // Validate non-empty input
    if network_range.trim().is_empty() {
        return Err(ScannerError::NetworkRangeInvalid(
            "Network range cannot be empty".to_string(),
        ));
    }

    if network_range.contains('/') {
        // CIDR notation
        MinerFactory::new()
            .with_subnet(network_range)
            .map_err(|e| ScannerError::NetworkRangeInvalid(format!("Invalid CIDR '{network_range}': {e}")))
    } else if network_range.contains('-') {
        // IP range notation
        MinerFactory::new()
            .with_range(network_range)
            .map_err(|e| ScannerError::NetworkRangeInvalid(format!("Invalid range '{network_range}': {e}")))
    } else {
        Err(ScannerError::NetworkRangeInvalid(format!(
            "Invalid format '{}'. Use CIDR (192.168.1.0/24) or range (192.168.1.1-100)",
            network_range
        )))
    }
}

/// Creates a MinerFactory with search filters applied.
///
/// # Errors
///
/// Returns `ScannerError::NetworkRangeInvalid` if the network range is invalid
pub fn create_configured_miner_factory(
    network_range: &str,
    config: &ScanConfig,
) -> Result<MinerFactory, ScannerError> {
    let mut factory = create_miner_factory(network_range)?;

    if let Some(ref makes) = config.search_makes {
        factory = factory.with_search_makes(makes.clone());
    }

    if let Some(ref firmwares) = config.search_firmwares {
        factory = factory.with_search_firmwares(firmwares.clone());
    }

    Ok(factory)
}

pub fn estimate_ip_count(network_range: &str) -> usize {
    match create_miner_factory(network_range) {
        Ok(factory) => factory.hosts().len(),
        Err(_) => 0,
    }
}
