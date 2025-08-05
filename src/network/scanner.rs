use asic_rs::data::device::{MinerFirmware, MinerMake};
use asic_rs::data::miner::MinerData;
use asic_rs::miners::factory::MinerFactory;
use iced::futures::{SinkExt, StreamExt};
use iced::stream;
use tokio::runtime::Runtime;

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
        miner: MinerData,
    },
    GroupScanCompleted {
        group_name: String,
        result: Result<(), String>,
    },
    AllScansCompleted,
}

pub struct Scanner {}

impl Scanner {
    /// Start scanning multiple groups simultaneously
    pub fn scan_multiple_groups(
        groups: Vec<(String, String, ScanConfig)>, // (group_name, range, config)
    ) -> iced::Subscription<ScannerMessage> {
        iced::Subscription::run_with_id(
            "multi_group_scanner".to_string(),
            Self::scan_multiple_groups_stream(groups),
        )
    }

    /// Create a stream that scans multiple groups sequentially with streaming
    fn scan_multiple_groups_stream(
        groups: Vec<(String, String, ScanConfig)>,
    ) -> impl iced::futures::Stream<Item = ScannerMessage> {
        stream::channel(100, |mut output| async move {
            let total_groups = groups.len();
            let mut completed_groups = 0;

            // Scan groups sequentially to avoid runtime issues
            for (group_name, network_range, config) in groups {
                // Perform real-time streaming scan for this group
                match Self::perform_realtime_scan(&network_range, &config, &mut output, &group_name)
                    .await
                {
                    Ok(()) => {
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

    /// Real-time streaming scan that sends miners immediately as they are discovered
    async fn perform_realtime_scan(
        network_range: &str,
        config: &ScanConfig,
        output: &mut iced::futures::channel::mpsc::Sender<ScannerMessage>,
        group_name: &str,
    ) -> Result<(), String> {
        // Create a separate Tokio runtime to avoid conflicts with Iced's runtime
        let rt = Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {e}"))?;

        // Create a channel to receive miners from the runtime
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        // Spawn the scanning task in the runtime
        let network_range = network_range.to_string();
        let config = config.clone();
        let handle = std::thread::spawn(move || {
            rt.block_on(async {
                // Build MinerFactory with configuration
                let mut factory = if network_range.contains('/') {
                    MinerFactory::new()
                        .with_subnet(&network_range)
                        .map_err(|e| format!("Invalid subnet: {e}"))?
                } else if network_range.contains('-') {
                    MinerFactory::new()
                        .with_range(&network_range)
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

                // Use the asic-rs scan_stream function for concurrent scanning
                let mut stream = factory
                    .scan_stream()
                    .map_err(|e| format!("Failed to create scan stream: {e}"))?;

                // Stream miners as they are discovered and send to channel
                while let Some(miner) = stream.next().await {
                    let miner_data = miner.get_data().await;
                    if tx.send(miner_data).is_err() {
                        // Channel closed, stop scanning
                        break;
                    }
                }

                Ok::<(), String>(())
            })
        });

        // Receive miners from the channel and forward to output immediately
        while let Some(miner) = rx.recv().await {
            if output
                .send(ScannerMessage::MinerDiscovered {
                    group_name: group_name.to_string(),
                    miner,
                })
                .await
                .is_err()
            {
                // Output channel closed, stop processing
                break;
            }
        }

        // Wait for the scanning thread to complete and check for errors
        match handle.join() {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Scanning thread panicked".to_string()),
        }
    }
}
