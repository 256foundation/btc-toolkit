use asic_rs::get_miner;
use asic_rs::miners::backends::traits::GetMinerData;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::task::JoinSet;
use tokio::time::timeout;

const ASIC_WEB_PORT: u16 = 80;
const TCP_CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ip_address: Ipv4Addr,
    pub miner: Option<String>,
    pub status: ScanStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScanStatus {
    Pending,
    Scanning,
    Found,
    NotFound,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum ScannerMessage {
    ScanStarted,
    ScanCompleted,
}

pub struct Scanner {
    results: Arc<Mutex<HashMap<Ipv4Addr, ScanResult>>>,
    running: Arc<Mutex<bool>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            results: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_scan(&self, ip_addresses: Vec<Ipv4Addr>) -> iced::Task<ScannerMessage> {
        if *self.running.lock().unwrap() {
            // Scan is already in progress
            // Consider returning a specific message or ensuring UI prevents this
            return iced::Task::none(); // Or some other appropriate Task
        }

        *self.running.lock().unwrap() = true;

        {
            let mut results_map = self.results.lock().unwrap();
            results_map.clear();
            for ip in &ip_addresses {
                results_map.insert(
                    *ip,
                    ScanResult {
                        ip_address: *ip,
                        miner: None,
                        status: ScanStatus::Pending,
                    },
                );
            }
        }

        let results_arc = Arc::clone(&self.results);
        let running_arc = Arc::clone(&self.running);
        // ip_addresses needs to be available to the block_on scope
        // Clone it if the original Vec is still needed elsewhere, or move it.
        // For iced::Task::perform, it's often moved.
        let ip_addresses_clone = ip_addresses; // Assuming ip_addresses is moved into the perform task

        iced::Task::perform(
            async move {
                // Create a dedicated Tokio runtime for the scanning operations.
                let rt = match Runtime::new() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to create Tokio runtime for scanner: {}", e);
                        // Propagate an error or complete with no results
                        // This requires a way to signal error if ScannerMessage doesn't support it.
                        // For now, let's assume it completes but results might reflect the failure implicitly.
                        *running_arc.lock().unwrap() = false; // Ensure running state is reset
                        return ScannerMessage::ScanCompleted; // Or a new error message type
                    }
                };

                // Use block_on to execute the scanning logic within this runtime.
                // ip_addresses_clone is moved into this block_on call.
                rt.block_on(async {
                    let mut tasks = JoinSet::new();
                    const MAX_CONCURRENT_SCANS: usize = 20;
                    // ip_queue now takes ownership from ip_addresses_clone
                    let mut ip_queue = ip_addresses_clone;

                    while !tasks.is_empty()
                        || (*running_arc.lock().unwrap() && !ip_queue.is_empty())
                    {
                        if *running_arc.lock().unwrap() {
                            while tasks.len() < MAX_CONCURRENT_SCANS && !ip_queue.is_empty() {
                                let ip_to_scan = ip_queue.remove(0);
                                {
                                    let mut results_map_guard = results_arc.lock().unwrap();
                                    if let Some(result_entry) =
                                        results_map_guard.get_mut(&ip_to_scan)
                                    {
                                        result_entry.status = ScanStatus::Scanning;
                                    }
                                }

                                tasks.spawn(async move {
                                    let ip_addr = IpAddr::V4(ip_to_scan);
                                    let sock_addr = SocketAddr::new(ip_addr, ASIC_WEB_PORT);
                                    let port_open = match timeout(
                                        TCP_CONNECT_TIMEOUT,
                                        TcpStream::connect(&sock_addr),
                                    )
                                    .await
                                    {
                                        Ok(Ok(_)) => true,
                                        _ => false,
                                    };

                                    let (status, miner_info) = if port_open {
                                        match get_miner(ip_addr).await {
                                            Ok(Some(miner_instance)) => {
                                                let miner_data = miner_instance.get_data().await;
                                                (
                                                    ScanStatus::Found,
                                                    Some(format!(
                                                        "{:?}",
                                                        miner_data.device_info.model
                                                    )),
                                                )
                                            }
                                            Ok(None) => (ScanStatus::NotFound, None),
                                            Err(e) => (
                                                ScanStatus::Error(format!(
                                                    "Discovery failed: {}",
                                                    e
                                                )),
                                                None,
                                            ),
                                        }
                                    } else {
                                        (ScanStatus::NotFound, None)
                                    };
                                    (ip_to_scan, status, miner_info)
                                });
                            }
                        }

                        if let Some(task_join_result) = tasks.join_next().await {
                            match task_join_result {
                                Ok((processed_ip, status, miner_info)) => {
                                    let mut results_map_guard = results_arc.lock().unwrap();
                                    if let Some(entry) = results_map_guard.get_mut(&processed_ip) {
                                        entry.status = status;
                                        entry.miner = miner_info;
                                    }
                                }
                                Err(join_err) => {
                                    eprintln!("A scan task panicked: {:?}", join_err);
                                }
                            }
                        } else if ip_queue.is_empty() && !*running_arc.lock().unwrap() {
                            break;
                        }
                    }
                }); // End of rt.block_on

                *running_arc.lock().unwrap() = false;
                ScannerMessage::ScanCompleted
            },
            |message| message,
        )
    }

    pub fn stop_scan(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    pub fn get_results(&self) -> HashMap<Ipv4Addr, ScanResult> {
        self.results.lock().unwrap().clone()
    }

    pub fn clear_results(&self) {
        let mut results_map = self.results.lock().unwrap();
        results_map.clear();
    }
}
