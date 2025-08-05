use asic_rs::data::device::{MinerFirmware, MinerMake};
use asic_rs::miners::factory::MinerFactory;
use iced::futures::SinkExt;
use iced::stream;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tokio::runtime::Runtime;

/// Represents a discovered miner with its details
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MinerInfo {
    pub ip: Ipv4Addr,
    pub model: String,
    pub make: Option<String>,
    pub firmware: Option<String>,
}

/// Configuration for scanner behavior
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ScanConfig {
    pub search_makes: Option<Vec<MinerMake>>,
    pub search_firmwares: Option<Vec<MinerFirmware>>,
}

#[derive(Debug, Clone)]
pub enum ScannerMessage {
    MinerDiscovered {
        group_name: String,
        miner: MinerInfo,
    },
    GroupScanCompleted {
        group_name: String,
        result: Result<(), String>,
    },
    AllScansCompleted,
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

    /// Start scanning the network range with proper streaming using Subscription
    pub fn scan(network_range: &str, config: ScanConfig) -> iced::Subscription<ScannerMessage> {
        let range = network_range.to_string();
        let config = config.clone();

        iced::Subscription::run_with_id(
            format!("scanner_{range}"),
            Self::scan_stream("Default".to_string(), range, config),
        )
    }

    /// Start scanning multiple groups simultaneously
    pub fn scan_multiple_groups(
        groups: Vec<(String, String, ScanConfig)>, // (group_name, range, config)
    ) -> iced::Subscription<ScannerMessage> {
        iced::Subscription::run_with_id(
            "multi_group_scanner".to_string(),
            Self::scan_multiple_groups_stream(groups),
        )
    }

    /// Create a stream that discovers miners and sends them as they are found
    fn scan_stream(
        group_name: String,
        network_range: String,
        config: ScanConfig,
    ) -> impl iced::futures::Stream<Item = ScannerMessage> {
        stream::channel(100, |mut output| async move {
            // Perform the scan to get all miners at once
            match Self::perform_scan(&network_range, &config) {
                Ok(miners) => {
                    if miners.is_empty() {
                        // No miners found, send completion immediately
                        let _ = output
                            .send(ScannerMessage::GroupScanCompleted {
                                group_name: group_name.clone(),
                                result: Ok(()),
                            })
                            .await;
                    } else {
                        // Send each miner immediately (streaming without delays)
                        for miner in miners.into_iter() {
                            // Send the discovered miner
                            if output
                                .send(ScannerMessage::MinerDiscovered {
                                    group_name: group_name.clone(),
                                    miner,
                                })
                                .await
                                .is_err()
                            {
                                // Channel closed, stop sending
                                break;
                            }
                        }

                        // Send completion after all miners are discovered
                        let _ = output
                            .send(ScannerMessage::GroupScanCompleted {
                                group_name: group_name.clone(),
                                result: Ok(()),
                            })
                            .await;
                    }
                }
                Err(e) => {
                    // Send error and complete
                    let _ = output
                        .send(ScannerMessage::GroupScanCompleted {
                            group_name: group_name.clone(),
                            result: Err(e),
                        })
                        .await;
                }
            }

            // Keep the stream alive until the subscription is dropped
            std::future::pending::<()>().await;
        })
    }

    /// Create a stream that scans multiple groups sequentially
    fn scan_multiple_groups_stream(
        groups: Vec<(String, String, ScanConfig)>,
    ) -> impl iced::futures::Stream<Item = ScannerMessage> {
        stream::channel(100, |mut output| async move {
            let total_groups = groups.len();
            let mut completed_groups = 0;

            // Scan groups sequentially to avoid runtime issues
            for (group_name, network_range, config) in groups {
                // Perform the scan for this group
                match Self::perform_scan(&network_range, &config) {
                    Ok(miners) => {
                        // Send discovered miners for this group
                        for miner in miners {
                            if output
                                .send(ScannerMessage::MinerDiscovered {
                                    group_name: group_name.clone(),
                                    miner,
                                })
                                .await
                                .is_err()
                            {
                                return; // Channel closed
                            }
                        }

                        // Send group completion
                        let _ = output
                            .send(ScannerMessage::GroupScanCompleted {
                                group_name: group_name.clone(),
                                result: Ok(()),
                            })
                            .await;
                    }
                    Err(e) => {
                        // Send group error
                        let _ = output
                            .send(ScannerMessage::GroupScanCompleted {
                                group_name: group_name.clone(),
                                result: Err(e),
                            })
                            .await;
                    }
                }

                completed_groups += 1;
            }

            // Send completion after all groups are done
            if completed_groups >= total_groups {
                let _ = output.send(ScannerMessage::AllScansCompleted).await;
            }

            // Keep the stream alive until the subscription is dropped
            std::future::pending::<()>().await;
        })
    }

    /// Internal scan implementation using asic-rs
    fn perform_scan(network_range: &str, config: &ScanConfig) -> Result<Vec<MinerInfo>, String> {
        // Create a Tokio runtime for asic-rs operations
        let rt = Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {e}"))?;

        rt.block_on(async {
            // Build MinerFactory with configuration
            let mut factory = if network_range.contains('/') {
                MinerFactory::new()
                    .with_subnet(network_range)
                    .map_err(|e| format!("Invalid subnet: {e}"))?
            } else if network_range.contains('-') {
                MinerFactory::new()
                    .with_range(network_range)
                    .map_err(|e| format!("Invalid range: {e}"))?
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
                .map_err(|e| format!("Scan failed: {e}"))?;

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
