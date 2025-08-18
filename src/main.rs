mod config;
mod main_view;
mod network;
mod network_config;
mod theme;

use crate::config::AppConfig;
use crate::main_view::{MainView, MainViewMessage};
use crate::network::scanner::{Scanner, ScannerMessage};
use crate::network_config::{NetworkConfig, NetworkConfigMessage};
use iced::{Element, Size, Subscription, Task, window};
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
        .theme(|_| theme::THEME)
        // Run with initial state
        .run_with(|| (BtcToolkit::new(), Task::none()))
}

// Enum to track which page is currently active
#[derive(Debug, Clone)]
enum Page {
    Main,
    NetworkConfig,
}

struct BtcToolkit {
    current_page: Page,
    main_view: MainView,
    network_config: NetworkConfig,
    active_scan: Option<Vec<network::scanner::ScanGroup>>,
    app_config: AppConfig,
}

impl BtcToolkit {
    fn new() -> Self {
        let app_config = AppConfig::load();
        let mut network_config = NetworkConfig::new();
        network_config.set_app_config(app_config.clone());

        let mut main_view = MainView::new();
        main_view.set_app_config(app_config.clone());

        Self {
            current_page: Page::Main,
            main_view,
            network_config,
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
    MainView(MainViewMessage),
    NetworkConfig(NetworkConfigMessage),
    Scanner(ScannerMessage), // Messages from the scanner
}

// Update function for the application
fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::MainView(message) => {
            match message.clone() {
                MainViewMessage::OpenNetworkConfig | MainViewMessage::AddGroup => {
                    state.current_page = Page::NetworkConfig;
                    Task::none()
                }

                MainViewMessage::StartScan => {
                    // Get enabled groups for scanning
                    let enabled_groups = state.app_config.get_enabled_groups();

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

                    let task = state.main_view.update(message);
                    task.map(BtcToolkitMessage::MainView)
                }

                MainViewMessage::StopScan => {
                    state.active_scan = None;
                    let task = state.main_view.update(message);
                    task.map(BtcToolkitMessage::MainView)
                }

                _ => {
                    let task = state.main_view.update(message);
                    task.map(BtcToolkitMessage::MainView)
                }
            }
        }

        BtcToolkitMessage::NetworkConfig(message) => {
            state.network_config.update(message.clone());

            match message {
                NetworkConfigMessage::Close => {
                    state.current_page = Page::Main;
                    Task::none()
                }
                NetworkConfigMessage::Save => {
                    // Update app config from network config
                    state.app_config = state.network_config.get_app_config().clone();
                    state.main_view.set_app_config(state.app_config.clone());
                    state.save_config();
                    state.current_page = Page::Main;
                    Task::none()
                }
                _ => Task::none(),
            }
        }

        BtcToolkitMessage::Scanner(scanner_msg) => {
            // Forward scanner messages to unified view
            match scanner_msg {
                ScannerMessage::MinerDiscovered { group_name, miner } => {
                    let _ = state
                        .main_view
                        .update(MainViewMessage::MinerFound { group_name, miner });
                }
                ScannerMessage::GroupScanCompleted { group_name, result } => match result {
                    Ok(()) => {
                        let _ = state
                            .main_view
                            .update(MainViewMessage::GroupCompleted(group_name));
                    }
                    Err(error) => {
                        let _ = state
                            .main_view
                            .update(MainViewMessage::GroupError { group_name, error });
                    }
                },
                ScannerMessage::AllScansCompleted => {
                    let _ = state.main_view.update(MainViewMessage::AllScansCompleted);
                    // Update config after all scans complete
                    state.app_config = state.main_view.get_app_config().clone();
                    state.save_config();
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
        Page::Main => state.main_view.view().map(BtcToolkitMessage::MainView),
        Page::NetworkConfig => state
            .network_config
            .view()
            .map(BtcToolkitMessage::NetworkConfig),
    }
}
