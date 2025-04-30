use asic_rs::get_miner;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::task::JoinSet;

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
    ScanProgress(Ipv4Addr, ScanStatus),
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
        // Mark scanner as running
        *self.running.lock().unwrap() = true;

        // Initialize results map with pending status
        {
            let mut results = self.results.lock().unwrap();
            results.clear(); // Clear previous results
            for ip in &ip_addresses {
                results.insert(
                    *ip,
                    ScanResult {
                        ip_address: *ip,
                        miner: None,
                        status: ScanStatus::Pending,
                    },
                );
            }
        }

        // Clone references for the task
        let results = Arc::clone(&self.results);
        let running = Arc::clone(&self.running);

        iced::Task::perform(
            async move {
                // Create a dedicated runtime for the scan
                let rt = Runtime::new().unwrap();

                rt.block_on(async {
                    // Create a set to track all spawned tasks
                    let mut tasks = JoinSet::new();

                    // Maximum number of concurrent scans
                    const MAX_CONCURRENT: usize = 20;

                    // Queue for tracking IPs being processed
                    let mut queue = ip_addresses;

                    // Process IPs until queue is empty
                    while !queue.is_empty() || !tasks.is_empty() {
                        // Fill the task set up to the concurrency limit
                        while tasks.len() < MAX_CONCURRENT && !queue.is_empty() {
                            if !*running.lock().unwrap() {
                                break;
                            }

                            let ip = queue.remove(0);

                            // Mark as scanning
                            {
                                let mut results = results.lock().unwrap();
                                if let Some(result) = results.get_mut(&ip) {
                                    result.status = ScanStatus::Scanning;
                                }
                            }

                            // Clone references for the task
                            let results = Arc::clone(&results);

                            // Spawn a task to scan this IP
                            tasks.spawn(async move {
                                let ip_addr = IpAddr::V4(ip);
                                let scan_result = get_miner(ip_addr).await;

                                // Update results
                                let mut results = results.lock().unwrap();
                                if let Some(result) = results.get_mut(&ip) {
                                    match scan_result {
                                        Ok(miner_info) => match miner_info {
                                            Some(miner_info) => {
                                                result.miner = Some(format!("{:?}", miner_info));
                                                result.status = ScanStatus::Found;
                                            }
                                            None => {
                                                result.status = ScanStatus::NotFound;
                                            }
                                        },
                                        Err(e) => {
                                            let error_msg = e.to_string();
                                            result.status = ScanStatus::Error(error_msg);
                                        }
                                    }
                                }
                                ip
                            });
                        }

                        // Stop if scanning was cancelled
                        if !*running.lock().unwrap() {
                            break;
                        }

                        // Wait for any task to complete
                        if let Some(result) = tasks.join_next().await {
                            // Just acknowledge completion - results are updated via shared state
                            let _ = result;
                        }
                    }
                });

                // Mark scanner as not running
                *running.lock().unwrap() = false;

                // Return completion message
                ScannerMessage::ScanCompleted
            },
            |msg| msg,
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
        self.results.lock().unwrap().clear();
    }
}
