pub mod scanner;

use asic_rs::miners::factory::MinerFactory;
use scanner::ScanConfig;

/// Create a MinerFactory from a network range string
pub fn create_miner_factory(network_range: &str) -> Result<MinerFactory, String> {
    if network_range.contains('/') {
        MinerFactory::new()
            .with_subnet(network_range)
            .map_err(|e| format!("Invalid subnet: {e}"))
    } else if network_range.contains('-') {
        MinerFactory::new()
            .with_range(network_range)
            .map_err(|e| format!("Invalid range: {e}"))
    } else {
        Err(
            "Invalid network range format. Use CIDR (192.168.1.0/24) or range (192.168.1.1-100)"
                .to_string(),
        )
    }
}

/// Create a configured MinerFactory with filters applied
pub fn create_configured_miner_factory(
    network_range: &str,
    config: &ScanConfig,
) -> Result<MinerFactory, String> {
    let mut factory = create_miner_factory(network_range)?;

    // Apply search filters if configured
    if let Some(ref makes) = config.search_makes {
        factory = factory.with_search_makes(makes.clone());
    }

    if let Some(ref firmwares) = config.search_firmwares {
        factory = factory.with_search_firmwares(firmwares.clone());
    }

    Ok(factory)
}

/// Estimate the number of IPs in a network range
pub fn estimate_ip_count(network_range: &str) -> usize {
    match create_miner_factory(network_range) {
        Ok(factory) => factory.hosts().len(),
        Err(_) => 0,
    }
}
