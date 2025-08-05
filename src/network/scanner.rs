use asic_rs::{
    data::{
        device::{MinerFirmware, MinerMake},
        miner::MinerData,
    },
    miners::factory::MinerFactory,
};
use iced::{
    futures::{SinkExt, StreamExt},
    stream,
};
use tokio::runtime::Runtime;

/// Configuration for scanner behavior
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

/// Represents a scan group with its network range and configuration
#[derive(Debug, Clone)]
pub struct ScanGroup {
    pub name: String,
    pub network_range: String,
    pub config: ScanConfig,
}

impl ScanGroup {
    pub fn new(
        name: impl Into<String>,
        network_range: impl Into<String>,
        config: ScanConfig,
    ) -> Self {
        Self {
            name: name.into(),
            network_range: network_range.into(),
            config,
        }
    }
}

pub struct Scanner;

impl Scanner {
    /// Start scanning multiple groups simultaneously
    pub fn scan_multiple_groups(groups: Vec<ScanGroup>) -> iced::Subscription<ScannerMessage> {
        iced::Subscription::run_with_id(
            "multi_group_scanner",
            Self::scan_multiple_groups_stream(groups),
        )
    }

    /// Create a stream that scans multiple groups sequentially with streaming
    fn scan_multiple_groups_stream(
        groups: Vec<ScanGroup>,
    ) -> impl iced::futures::Stream<Item = ScannerMessage> {
        stream::channel(100, |mut output| async move {
            let total_groups = groups.len();

            // Scan groups sequentially to avoid runtime issues
            for (index, group) in groups.into_iter().enumerate() {
                // Send group completion based on scan result
                let result = Self::perform_realtime_scan(
                    &group.network_range,
                    &group.config,
                    &mut output,
                    &group.name,
                )
                .await;

                let _ = output
                    .send(ScannerMessage::GroupScanCompleted {
                        group_name: group.name,
                        result,
                    })
                    .await;

                // Send completion after all groups are done
                if index + 1 == total_groups {
                    let _ = output.send(ScannerMessage::AllScansCompleted).await;
                }
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
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MinerData>();

        // Spawn the scanning task in the runtime
        let network_range = network_range.to_owned();
        let config = config.clone();
        let group_name = group_name.to_owned();

        let handle = std::thread::spawn(move || {
            rt.block_on(async move { Self::scan_network(&network_range, &config, tx).await })
        });

        // Receive miners from the channel and forward to output immediately
        while let Some(miner) = rx.recv().await {
            if output
                .send(ScannerMessage::MinerDiscovered {
                    group_name: group_name.clone(),
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
        handle
            .join()
            .map_err(|_| "Scanning thread panicked".to_string())?
    }

    /// Scan a network range and send discovered miners through the channel
    async fn scan_network(
        network_range: &str,
        config: &ScanConfig,
        tx: tokio::sync::mpsc::UnboundedSender<MinerData>,
    ) -> Result<(), String> {
        // Build MinerFactory with configuration
        let factory = Self::create_factory(network_range, config)?;

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

        Ok(())
    }

    /// Create and configure a MinerFactory based on network range and config
    fn create_factory(network_range: &str, config: &ScanConfig) -> Result<MinerFactory, String> {
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

        Ok(factory)
    }
}
