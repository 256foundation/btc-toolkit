mod dashboard;
mod network;
mod network_config;
mod scanning_view;

use crate::dashboard::{Dashboard, DashboardMessage};
use crate::network::scanner::{Scanner, ScannerMessage};
use crate::network_config::{NetworkConfig, NetworkConfigMessage};
use crate::scanning_view::{ScanningMessage, ScanningView};
use iced::{Element, Size, window};
use mimalloc::MiMalloc;

// http://github.com/microsoft/mimalloc
// https://github.com/purpleprotocol/mimalloc_rust
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> iced::Result {
    // Create application with title, update function, and view function
    iced::application("BTC Toolkit", update, view)
        // Configure window with custom settings
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        // Run with initial state
        .run_with(|| (BtcToolkit::new(), iced::Task::none()))
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
}

impl BtcToolkit {
    fn new() -> Self {
        Self {
            current_page: Page::Dashboard,
            main_page: Dashboard::new(),
            network_config: NetworkConfig::new(),
            scanning_view: None,
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
fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> iced::Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::Dashboard(message) => {
            match message {
                DashboardMessage::OpenNetworkConfig => {
                    state.current_page = Page::NetworkConfig;
                    iced::Task::none()
                }
                DashboardMessage::NavigateToScanning(ip_range, total_ips) => {
                    // Create scanning view and switch to it
                    state.scanning_view = Some(ScanningView::new(total_ips));
                    state.current_page = Page::Scanning;

                    // Start the actual scan
                    let scan_config = state.network_config.get_scan_config().clone();
                    Scanner::scan(&ip_range, scan_config).map(BtcToolkitMessage::Scanner)
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
                    iced::Task::none()
                }
                NetworkConfigMessage::Save => {
                    // Pass updated config to dashboard
                    state
                        .main_page
                        .set_network_config(state.network_config.clone());
                    state.current_page = Page::Dashboard;
                    iced::Task::none()
                }
                other => {
                    // Forward other network config messages to update the config
                    state.network_config.update(other);
                    iced::Task::none()
                }
            }
        }

        BtcToolkitMessage::Scanner(scanner_msg) => {
            // Forward scanner messages to scanning view if active
            if let Some(ref mut scanning_view) = state.scanning_view {
                match scanner_msg {
                    ScannerMessage::MinerDiscovered(miner) => {
                        scanning_view.update(ScanningMessage::MinerFound(miner));
                    }
                    ScannerMessage::ScanCompleted(Ok(())) => {
                        scanning_view.update(ScanningMessage::ScanCompleted);
                    }
                    ScannerMessage::ScanCompleted(Err(error)) => {
                        scanning_view.update(ScanningMessage::ScanError(error));
                    }
                }
            }
            iced::Task::none()
        }
        BtcToolkitMessage::Scanning(message) => {
            if let Some(ref mut scanning_view) = state.scanning_view {
                scanning_view.update(message.clone());

                match message {
                    ScanningMessage::BackToDashboard => {
                        // Copy discovered miners back to dashboard
                        let discovered_miners = scanning_view.get_discovered_miners().clone();
                        state.main_page.set_scan_results(discovered_miners);

                        // Return to dashboard
                        state.current_page = Page::Dashboard;
                        state.scanning_view = None;
                    }
                    ScanningMessage::StopScan => {
                        // Stop scanning (TODO: implement proper cancellation)
                    }
                    _ => {}
                }
            }
            iced::Task::none()
        }
    }
}

// View function for the application
fn view(state: &BtcToolkit) -> Element<BtcToolkitMessage> {
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
