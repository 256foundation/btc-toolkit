use asic_rs::data::device::{MinerFirmware, MinerMake};
use asic_rs::miners::factory::MinerFactory;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tokio::runtime::Runtime;

/// Represents a discovered miner with its details
#[derive(Debug, Clone)]
pub struct MinerInfo {
    pub ip: Ipv4Addr,
    pub model: String,
    pub make: Option<String>,
    pub firmware: Option<String>,
}

/// Configuration for scanner behavior
#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub search_makes: Option<Vec<MinerMake>>,
    pub search_firmwares: Option<Vec<MinerFirmware>>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            search_makes: None,
            search_firmwares: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScannerMessage {
    MinerDiscovered(MinerInfo),
    ScanCompleted(Result<(), String>),
}

/// Simplified scanner that wraps asic-rs MinerFactory
pub struct Scanner {
    results: HashMap<Ipv4Addr, MinerInfo>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    /// Start scanning the network range - simplified approach
    pub fn scan(network_range: &str, config: ScanConfig) -> iced::Task<ScannerMessage> {
        let range = network_range.to_string();

        iced::Task::perform(
            async move { Self::perform_simple_scan(&range, &config).await },
            |result| result,
        )
    }

    /// Simple scan implementation that returns a single result
    async fn perform_simple_scan(network_range: &str, config: &ScanConfig) -> ScannerMessage {
        match Self::perform_scan(network_range, config) {
            Ok(miners) => {
                if miners.is_empty() {
                    ScannerMessage::ScanCompleted(Ok(()))
                } else {
                    // For now, just return the first miner found
                    // TODO: Implement true streaming
                    ScannerMessage::MinerDiscovered(miners[0].clone())
                }
            }
            Err(e) => ScannerMessage::ScanCompleted(Err(e)),
        }
    }

    /// Internal scan implementation using asic-rs
    fn perform_scan(network_range: &str, config: &ScanConfig) -> Result<Vec<MinerInfo>, String> {
        // Create a Tokio runtime for asic-rs operations
        let rt = Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;

        rt.block_on(async {
            // Build MinerFactory with configuration
            let mut factory = if network_range.contains('/') {
                MinerFactory::new()
                    .with_subnet(network_range)
                    .map_err(|e| format!("Invalid subnet: {}", e))?
            } else if network_range.contains('-') {
                MinerFactory::new()
                    .with_range(network_range)
                    .map_err(|e| format!("Invalid range: {}", e))?
            } else {
                return Err("Invalid network range format. Use CIDR (192.168.1.0/24) or range (192.168.1.1-100)".to_string());
            };

            // Apply search filters if configured
            if let Some(ref makes) = config.search_makes {
                factory = factory.with_search_makes(makes.clone());
            }

            if let Some(ref firmwares) = config.search_firmwares {
                factory = factory.with_search_firmwares(firmwares.clone());
            }

            // Perform the scan
            let miners = factory
                .scan()
                .await
                .map_err(|e| format!("Scan failed: {}", e))?;

            // Extract miner information
            let mut results = Vec::new();
            for miner in miners {
                let miner_data = miner.get_data().await;
                let ip = miner_data
                    .ip
                    .to_string()
                    .parse::<Ipv4Addr>()
                    .map_err(|_| "Invalid IP address format")?;

                results.push(MinerInfo {
                    ip,
                    model: format!("{:?}", miner_data.device_info.model),
                    make: Some(format!("{:?}", miner_data.device_info.make)),
                    firmware: Some(format!("{:?}", miner_data.device_info.firmware)),
                });
            }

            Ok(results)
        })
    }

    // State management methods (simplified)
    pub fn get_results(&self) -> &HashMap<Ipv4Addr, MinerInfo> {
        &self.results
    }

    pub fn set_results_from_map(&mut self, results: HashMap<Ipv4Addr, MinerInfo>) {
        self.results = results;
    }
}
