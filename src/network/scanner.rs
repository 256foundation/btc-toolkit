use std::sync::{Arc, atomic::AtomicUsize};
use std::time::{Duration, Instant};

use crate::errors::{ScannerError, ScannerResult};
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
// Tokio runtime is now shared via iced's tokio feature flag

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ScanConfig {
    pub search_makes: Option<Vec<MinerMake>>,
    pub search_firmwares: Option<Vec<MinerFirmware>>,
}

impl std::hash::Hash for ScanConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on JSON serialization for simplicity
        if let Ok(json) = serde_json::to_string(self) {
            json.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
struct ThrottledProgress {
    group_name: String,
    total_ips: usize,
    scanned_count: usize,
}

/// Calculates an appropriate buffer size for the channel based on estimated IP count.
///
/// Uses a dynamic buffer size to balance memory usage and performance:
/// - Minimum: 50 (for small networks)
/// - Maximum: 1000 (to prevent excessive memory usage)
/// - Formula: 50 + (estimated_ips / 10)
const fn calculate_buffer_size(estimated_ips: usize) -> usize {
    const MIN_BUFFER: usize = 50;
    const MAX_BUFFER: usize = 1000;
    const DIVISOR: usize = 10;

    let calculated = MIN_BUFFER + estimated_ips / DIVISOR;

    if calculated < MIN_BUFFER {
        MIN_BUFFER
    } else if calculated > MAX_BUFFER {
        MAX_BUFFER
    } else {
        calculated
    }
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
    IpScanned {
        group_name: String,
        total_ips: usize,
        scanned_count: usize,
    },
    GroupScanCompleted {
        group_name: String,
        result: Result<(), String>,
    },
    AllScansCompleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    pub fn scan_multiple_groups(groups: Vec<ScanGroup>) -> iced::Subscription<ScannerMessage> {
        iced::Subscription::run_with(groups, Self::scan_multiple_groups_stream)
    }

    fn scan_multiple_groups_stream(
        groups: &Vec<ScanGroup>,
    ) -> iced::futures::stream::BoxStream<'static, ScannerMessage> {
        use iced::futures::StreamExt;
        let groups = groups.clone();
        let total_estimated_ips: usize = groups
            .iter()
            .map(|group| super::estimate_ip_count(&group.network_range))
            .sum();

        let buffer_size = calculate_buffer_size(total_estimated_ips);

        stream::channel(
            buffer_size,
            |mut output: iced::futures::channel::mpsc::Sender<ScannerMessage>| async move {
                use future::join_all;

                let total_groups = groups.len();

                if total_groups == 0 {
                    let _ = output.send(ScannerMessage::AllScansCompleted).await;
                    std::future::pending::<()>().await;
                    return;
                }

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
                        .await
                        .map_err(|e| e.to_string());

                        let _ = output_clone
                            .send(ScannerMessage::GroupScanCompleted { group_name, result })
                            .await;
                    }
                });

                join_all(scan_futures).await;

                let _ = output.send(ScannerMessage::AllScansCompleted).await;

                std::future::pending::<()>().await;
            },
        )
        .boxed()
    }

    async fn perform_realtime_scan(
        network_range: &str,
        config: &ScanConfig,
        output: &mut iced::futures::channel::mpsc::Sender<ScannerMessage>,
        group_name: &str,
    ) -> ScannerResult<()> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MinerData>();
        let (progress_tx, mut progress_rx) =
            tokio::sync::mpsc::unbounded_channel::<ThrottledProgress>();

        // Clone only what we need for the async task
        let network_range = network_range.to_string();
        let config = config.clone();
        let group_name = group_name.to_string();
        let group_name_for_task = group_name.clone();

        // Spawn scan task on shared tokio runtime
        // This runs concurrently without blocking the UI thread
        let scan_handle = tokio::spawn(async move {
            Self::scan_network(
                &network_range,
                &config,
                tx,
                progress_tx,
                group_name_for_task,
            )
            .await
        });

        let mut last_progress_time = Instant::now();
        const PROGRESS_THROTTLE_MS: u64 = 100; // Throttle to every 100ms

        loop {
            tokio::select! {
                miner_opt = rx.recv() => {
                    match miner_opt {
                        Some(miner) => {
                            if output
                                .send(ScannerMessage::MinerDiscovered {
                                    group_name: group_name.to_owned(),
                                    miner,
                                })
                                .await
                                .is_err()
                            {
                                return Err(ScannerError::ChannelClosed);
                            }
                        }
                        None => {}
                    }
                }

                progress_opt = progress_rx.recv() => {
                    match progress_opt {
                        Some(throttled_progress) => {
                            let now = Instant::now();
                            // Throttle progress updates to avoid UI flooding
                            if now.duration_since(last_progress_time) >= Duration::from_millis(PROGRESS_THROTTLE_MS) {
                                let progress_msg = ScannerMessage::IpScanned {
                                    group_name: throttled_progress.group_name,
                                    total_ips: throttled_progress.total_ips,
                                    scanned_count: throttled_progress.scanned_count,
                                };

                                if output.send(progress_msg).await.is_err() {
                                    return Err(ScannerError::ChannelClosed);
                                }
                                last_progress_time = now;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        // Wait for the background scan task to complete
        scan_handle.await.map_err(|e| {
            ScannerError::ThreadError(format!("Background scan task failed: {}", e))
        })??;

        Ok(())
    }

    async fn scan_network(
        network_range: &str,
        config: &ScanConfig,
        tx: tokio::sync::mpsc::UnboundedSender<MinerData>,
        progress_tx: tokio::sync::mpsc::UnboundedSender<ThrottledProgress>,
        group_name: String,
    ) -> ScannerResult<()> {
        let factory = super::create_configured_miner_factory(network_range, config)?;
        let total_ips = factory.hosts().len();

        let stream = factory.scan_stream_with_ip();

        let scanned_count = Arc::new(AtomicUsize::new(0));

        // Scan all IPs concurrently with no limit
        stream
            .for_each_concurrent(None, move |(_ip, miner)| {
                let tx = tx.clone(); // Much cheaper than Arc<Mutex>
                let progress_tx = progress_tx.clone();
                let scanned_count = scanned_count.clone();
                let group_name = group_name.clone();

                async move {
                    let current_count =
                        scanned_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;

                    let _ = progress_tx.send(ThrottledProgress {
                        group_name: group_name.clone(),
                        total_ips,
                        scanned_count: current_count,
                    });

                    if let Some(miner) = miner {
                        let miner_data = get_partial_data(miner).await;
                        let _ = tx.send(miner_data);
                    }
                }
            })
            .await;

        Ok(())
    }
}
