mod config;
mod dashboard;
mod network;
mod network_config;
mod scanning_view;
mod theme;

use crate::config::AppConfig;
use crate::dashboard::{Dashboard, DashboardMessage};
use crate::network::estimate_ip_count;
use crate::network::scanner::{Scanner, ScannerMessage};
use crate::network_config::{NetworkConfig, NetworkConfigMessage};
use crate::scanning_view::{ScanningMessage, ScanningView};
use iced::{window, Element, Size, Subscription, Task};
use mimalloc::MiMalloc;

// http://github.com/microsoft/mimalloc
// https://github.com/purpleprotocol/mimalloc_rust
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> iced::Result {
    // Create application with title, update function, and view function
    iced::application("BTC Toolkit", update, view)
        // Add subscription for streaming scanner events
        .subscription(subscription)
        // Configure window with custom settings optimized for ASIC farm management
        .window(window::Settings {
            size: Size::new(1200.0, 800.0), // Larger default size for better data visibility
            position: window::Position::Centered,
            min_size: Some(Size::new(1000.0, 650.0)), // Ensure minimum usability
            ..window::Settings::default()
        })
        // Apply Bitcoin-inspired theme
        .theme(|_| theme::btc_theme())
        // Run with initial state
        .run_with(|| (BtcToolkit::new(), Task::none()))
}

// Enum to track which page is currently active
#[derive(Debug, Clone)]
enum Page {
    Dashboard,
    NetworkConfig,
    Scanning,
}

struct BtcToolkit {
    current_page: Page,
    main_page: Dashboard,
    network_config: NetworkConfig,
    scanning_view: Option<ScanningView>,
    active_scan: Option<Vec<network::scanner::ScanGroup>>,
    app_config: AppConfig,
}

impl BtcToolkit {
    fn new() -> Self {
        let app_config = AppConfig::load();
        let mut network_config = NetworkConfig::new();
        network_config.set_app_config(app_config.clone());

        let mut main_page = Dashboard::new();
        main_page.set_app_config(app_config.clone());

        Self {
            current_page: Page::Dashboard,
            main_page,
            network_config,
            scanning_view: None,
            active_scan: None,
            app_config,
        }
    }

    fn save_config(&self) {
        if let Err(e) = self.app_config.save() {
            eprintln!("Failed to save config: {e}");
        }
    }
}

#[derive(Debug, Clone)]
enum BtcToolkitMessage {
    Dashboard(DashboardMessage),
    NetworkConfig(NetworkConfigMessage),
    Scanning(ScanningMessage),
    Scanner(ScannerMessage), // Messages from the scanner
}

// Update function for the application
fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::Dashboard(message) => {
            match message {
                DashboardMessage::OpenNetworkConfig => {
                    state.current_page = Page::NetworkConfig;
                    Task::none()
                }
                DashboardMessage::NavigateToScanning => {
                    // Get enabled groups for scanning
                    let enabled_groups = state.app_config.get_enabled_groups();
                    let total_ips: usize = enabled_groups
                        .iter()
                        .map(|group| estimate_ip_count(&group.network_range))
                        .sum();

                    // Create scanning view and switch to it
                    state.scanning_view = Some(ScanningView::new_multi_group(
                        enabled_groups.len(),
                        total_ips,
                    ));
                    state.current_page = Page::Scanning;

                    // Set active scan for subscription - collect all enabled groups
                    let active_scans: Vec<network::scanner::ScanGroup> = enabled_groups
                        .into_iter()
                        .map(|group| {
                            network::scanner::ScanGroup::new(
                                group.name.clone(),
                                group.network_range.clone(),
                                group.scan_config.clone(),
                            )
                        })
                        .collect();

                    state.active_scan = if active_scans.is_empty() {
                        None
                    } else {
                        Some(active_scans)
                    };

                    Task::none()
                }
                _ => {
                    let task = state.main_page.update(message);
                    task.map(BtcToolkitMessage::Dashboard)
                }
            }
        }
        BtcToolkitMessage::NetworkConfig(message) => {
            state.network_config.update(message.clone());

            match message {
                NetworkConfigMessage::Close => {
                    state.current_page = Page::Dashboard;
                    Task::none()
                }
                NetworkConfigMessage::Save => {
                    // Update app config from network config
                    state.app_config = state.network_config.get_app_config().clone();
                    state.main_page.set_app_config(state.app_config.clone());
                    state.save_config();
                    state.current_page = Page::Dashboard;
                    Task::none()
                }
                _ => Task::none(),
            }
        }

        BtcToolkitMessage::Scanner(scanner_msg) => {
            // Forward scanner messages to scanning view if active
            if let Some(ref mut scanning_view) = state.scanning_view {
                match scanner_msg {
                    ScannerMessage::MinerDiscovered { group_name, miner } => {
                        scanning_view.update(ScanningMessage::MinerFound { group_name, miner });
                    }
                    ScannerMessage::GroupScanCompleted { group_name, result } => match result {
                        Ok(()) => {
                            scanning_view.update(ScanningMessage::GroupCompleted(group_name));
                        }
                        Err(error) => {
                            scanning_view.update(ScanningMessage::GroupError { group_name, error });
                        }
                    },
                    ScannerMessage::AllScansCompleted => {
                        scanning_view.update(ScanningMessage::AllScansCompleted);
                    }
                }
            }
            Task::none()
        }
        BtcToolkitMessage::Scanning(message) => {
            if let Some(ref mut scanning_view) = state.scanning_view {
                scanning_view.update(message.clone());

                match message {
                    ScanningMessage::BackToDashboard => {
                        // Copy discovered miners back to dashboard and config
                        let discovered_miners_by_group =
                            scanning_view.get_discovered_miners_by_group();

                        // Store results in app config
                        for (group_name, miners) in discovered_miners_by_group {
                            state.app_config.store_scan_results(&group_name, miners);
                        }

                        // Update dashboard
                        state.main_page.set_app_config(state.app_config.clone());

                        // Save config with results
                        state.save_config();

                        // Return to dashboard and stop scanning
                        state.current_page = Page::Dashboard;
                        state.scanning_view = None;
                        state.active_scan = None;
                    }
                    ScanningMessage::StopScan => {
                        // Stop scanning by clearing active scan
                        state.active_scan = None;
                    }
                    _ => {}
                }
            }
            Task::none()
        }
    }
}

// Subscription function for handling ongoing scanner streams
fn subscription(state: &BtcToolkit) -> Subscription<BtcToolkitMessage> {
    if let Some(ref active_scans) = state.active_scan {
        Scanner::scan_multiple_groups(active_scans.clone()).map(BtcToolkitMessage::Scanner)
    } else {
        Subscription::none()
    }
}

// View function for the application
fn view(state: &BtcToolkit) -> Element<'_, BtcToolkitMessage> {
    match state.current_page {
        Page::Dashboard => state.main_page.view().map(BtcToolkitMessage::Dashboard),
        Page::NetworkConfig => state
            .network_config
            .view()
            .map(BtcToolkitMessage::NetworkConfig),
        Page::Scanning => {
            if let Some(ref scanning_view) = state.scanning_view {
                scanning_view.view().map(BtcToolkitMessage::Scanning)
            } else {
                // Fallback to dashboard if no scanning view
                state.main_page.view().map(BtcToolkitMessage::Dashboard)
            }
        }
    }
}
