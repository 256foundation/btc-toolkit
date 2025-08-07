use asic_rs::{
    data::{
        device::{MinerFirmware, MinerMake},
        miner::MinerData,
    },
    miners::{backends::traits::GetMinerData, data::DataField},
};
use iced::{
    futures::{SinkExt, StreamExt, future},
    stream,
};
use tokio::runtime::Runtime;

/// Scanner configuration with optional filters
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ScanConfig {
    pub search_makes: Option<Vec<MinerMake>>,
    pub search_firmwares: Option<Vec<MinerFirmware>>,
}

/// Calculate adaptive buffer size (50-1000 range)
fn calculate_buffer_size(estimated_ips: usize) -> usize {
    (50 + estimated_ips / 10).min(1000).max(50)
}

async fn get_partial_data(miner: Box<dyn GetMinerData>) -> MinerData {
    let mut collector = miner.get_collector();
    let data = collector
        .collect(&[DataField::Mac, DataField::FirmwareVersion])
        .await;

    miner.parse_data(data)
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

/// Network scan group with range and filters
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
    /// Scan multiple groups in parallel
    pub fn scan_multiple_groups(groups: Vec<ScanGroup>) -> iced::Subscription<ScannerMessage> {
        iced::Subscription::run_with_id(
            "multi_group_scanner",
            Self::scan_multiple_groups_stream(groups),
        )
    }

    /// Create parallel scanning stream with adaptive buffering
    fn scan_multiple_groups_stream(
        groups: Vec<ScanGroup>,
    ) -> impl iced::futures::Stream<Item = ScannerMessage> {
        // Size buffer based on total IP count
        let total_estimated_ips: usize = groups
            .iter()
            .map(|group| super::estimate_ip_count(&group.network_range))
            .sum();

        let buffer_size = calculate_buffer_size(total_estimated_ips);

        stream::channel(buffer_size, |mut output| async move {
            use future::join_all;

            let total_groups = groups.len();

            if total_groups == 0 {
                let _ = output.send(ScannerMessage::AllScansCompleted).await;
                std::future::pending::<()>().await;
                return;
            }

            // Spawn parallel scan tasks
            let scan_futures = groups.into_iter().map(|group| {
                let mut output_clone = output.clone();
                let group_name = group.name.clone();

                async move {
                    let result = Self::perform_realtime_scan(
                        &group.network_range,
                        &group.config,
                        &mut output_clone,
                        &group.name,
                    )
                    .await;

                    // Report group completion
                    let _ = output_clone
                        .send(ScannerMessage::GroupScanCompleted { group_name, result })
                        .await;
                }
            });

            // Execute parallel scans
            join_all(scan_futures).await;

            // Signal all scans complete
            let _ = output.send(ScannerMessage::AllScansCompleted).await;

            // Keep stream alive for subscription lifecycle
            std::future::pending::<()>().await;
        })
    }

    /// Real-time scan with dedicated runtime bridge
    async fn perform_realtime_scan(
        network_range: &str,
        config: &ScanConfig,
        output: &mut iced::futures::channel::mpsc::Sender<ScannerMessage>,
        group_name: &str,
    ) -> Result<(), String> {
        // Channel for async miner streaming
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MinerData>();

        // Dedicated runtime for Tokio operations (Iced runs outside Tokio context)
        let rt = Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {e}"))?;

        let network_range = network_range.to_owned();
        let config = config.clone();

        // Thread bridge to Tokio runtime
        let scan_handle = std::thread::spawn(move || {
            rt.block_on(async move { Self::scan_network(&network_range, &config, tx).await })
        });

        // Forward discovered miners to output stream
        while let Some(miner) = rx.recv().await {
            if output
                .send(ScannerMessage::MinerDiscovered {
                    group_name: group_name.to_owned(),
                    miner,
                })
                .await
                .is_err()
            {
                // Output closed, stop forwarding
                break;
            }
        }

        // Wait for scan completion
        scan_handle
            .join()
            .map_err(|_| "Scanning thread panicked".to_string())?
    }

    /// Network scan with miner discovery streaming
    async fn scan_network(
        network_range: &str,
        config: &ScanConfig,
        tx: tokio::sync::mpsc::UnboundedSender<MinerData>,
    ) -> Result<(), String> {
        // Create configured factory
        let factory = super::create_configured_miner_factory(network_range, config)?;

        // Stream concurrent IP scans
        let mut stream = factory
            .scan_stream_with_ip()
            .map_err(|e| format!("Failed to create scan stream: {e}"))?;

        // Forward discovered miners
        while let Some(result) = stream.next().await {
            let (_miner_ip, maybe_miner) = result;
            match maybe_miner {
                Some(miner) => {
                    // Use get_partial_data to collect minimal fields
                    let miner_data = get_partial_data(miner).await;
                    if tx.send(miner_data).is_err() {
                        // Channel closed, stop scan
                        break;
                    }
                }
                None => {}
            }
        }

        Ok(())
    }
}
