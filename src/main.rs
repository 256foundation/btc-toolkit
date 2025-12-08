mod config;
mod device_detail_view;
mod errors;
mod main_view;
mod network;
mod network_config;
mod sorting;
mod theme;
mod ui_helpers;

use crate::config::AppConfig;
use crate::device_detail_view::{DeviceDetailMessage, DeviceDetailView};
use crate::main_view::{MainView, MainViewMessage};
use crate::network::scanner::{Scanner, ScannerMessage};
use crate::network_config::{NetworkConfig, NetworkConfigMessage};
use iced::{Element, Size, Subscription, Task, Theme, window};
use mimalloc::MiMalloc;
use std::net::IpAddr;

// http://github.com/microsoft/mimalloc
// https://github.com/purpleprotocol/mimalloc_rust
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Main entry point
///
/// Note: We don't use #[tokio::main] because iced with the "tokio" feature flag
/// manages its own tokio runtime internally. Using #[tokio::main] would create
/// a nested runtime situation that causes panics during shutdown.
fn main() -> iced::Result {
    iced::application(BtcToolkit::boot, update, view)
        .subscription(subscription)
        .window(window::Settings {
            size: Size::new(1200.0, 800.0),
            position: window::Position::Centered,
            min_size: Some(Size::new(1000.0, 650.0)),
            ..window::Settings::default()
        })
        .theme(BtcToolkit::theme)
        .title("BTC Toolkit")
        .run()
}

#[derive(Debug, Clone)]
enum Page {
    Main,
    NetworkConfig,
    DeviceDetail(IpAddr),
}

struct BtcToolkit {
    current_page: Page,
    main_view: MainView,
    network_config: NetworkConfig,
    device_detail_view: Option<DeviceDetailView>,
    active_scan: Option<Vec<network::scanner::ScanGroup>>,
    app_config: AppConfig,
}

impl BtcToolkit {
    fn boot() -> (Self, Task<BtcToolkitMessage>) {
        let app_config = AppConfig::load();
        let mut network_config = NetworkConfig::new();
        network_config.set_app_config(app_config.clone());

        let mut main_view = MainView::new();
        main_view.set_app_config(app_config.clone());

        (
            Self {
                current_page: Page::Main,
                main_view,
                network_config,
                device_detail_view: None,
                active_scan: None,
                app_config,
            },
            Task::none(),
        )
    }

    fn theme(&self) -> Theme {
        theme::theme()
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
    DeviceDetail(DeviceDetailMessage),
    Scanner(ScannerMessage),
}

fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::MainView(message) => match message.clone() {
            MainViewMessage::OpenNetworkConfig | MainViewMessage::AddGroup => {
                state.current_page = Page::NetworkConfig;
                Task::none()
            }

            MainViewMessage::OpenDeviceDetail(ip) => {
                // Set loading state and trigger full data fetch
                state.device_detail_view = Some(DeviceDetailView::new_loading(IpAddr::V4(ip)));
                state.current_page = Page::DeviceDetail(IpAddr::V4(ip));

                // Fetch full miner data
                // Note: With iced's tokio feature enabled, Task::perform runs on the
                // shared tokio runtime, so we use the async version directly
                Task::perform(
                    network::full_fetch::fetch_full_miner_data_async(IpAddr::V4(ip)),
                    |result| {
                        BtcToolkitMessage::DeviceDetail(DeviceDetailMessage::DataFetched(result))
                    },
                )
            }

            MainViewMessage::StartScan => {
                let enabled_groups = state.app_config.get_enabled_groups();

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
        },

        BtcToolkitMessage::NetworkConfig(message) => {
            state.network_config.update(message.clone());

            match message {
                NetworkConfigMessage::Close => {
                    state.current_page = Page::Main;
                    Task::none()
                }
                NetworkConfigMessage::Save => {
                    state.app_config = state.network_config.get_app_config().clone();
                    state.main_view.set_app_config(state.app_config.clone());
                    state.save_config();
                    state.current_page = Page::Main;
                    Task::none()
                }
                _ => Task::none(),
            }
        }

        BtcToolkitMessage::DeviceDetail(message) => {
            match message {
                DeviceDetailMessage::Back => {
                    state.current_page = Page::Main;
                    state.device_detail_view = None;
                    Task::none()
                }
                DeviceDetailMessage::DataFetched(result) => {
                    // Update the device detail view with fetched data
                    if let Some(ref mut view) = state.device_detail_view {
                        view.update_with_data(result);
                    }
                    Task::none()
                }
                DeviceDetailMessage::OpenInBrowser => {
                    // Extract IP from current page and open in browser
                    if let Page::DeviceDetail(ip) = state.current_page {
                        let url = format!("http://{}", ip);
                        if let Err(e) = opener::open(&url) {
                            eprintln!("Failed to open URL {}: {}", url, e);
                        }
                    }
                    Task::none()
                }
                DeviceDetailMessage::Restart
                | DeviceDetailMessage::SetPowerLimit
                | DeviceDetailMessage::ToggleFaultLight => {
                    // These would require implementing the control features from asic-rs
                    // For now, just return Task::none()
                    Task::none()
                }
            }
        }

        BtcToolkitMessage::Scanner(scanner_msg) => {
            match scanner_msg {
                ScannerMessage::MinerDiscovered { group_name, miner } => {
                    let _ = state
                        .main_view
                        .update(MainViewMessage::MinerFound { group_name, miner });
                }
                ScannerMessage::IpScanned {
                    group_name,
                    total_ips,
                    scanned_count,
                } => {
                    let _ = state.main_view.update(MainViewMessage::IpScanned {
                        group_name,
                        total_ips,
                        scanned_count,
                    });
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
                    state.app_config = state.main_view.get_app_config().clone();
                    state.save_config();
                }
            }
            Task::none()
        }
    }
}

fn subscription(state: &BtcToolkit) -> Subscription<BtcToolkitMessage> {
    if let Some(ref active_scans) = state.active_scan {
        Scanner::scan_multiple_groups(active_scans.clone()).map(BtcToolkitMessage::Scanner)
    } else {
        Subscription::none()
    }
}

fn view(state: &BtcToolkit) -> Element<'_, BtcToolkitMessage> {
    match &state.current_page {
        Page::Main => state.main_view.view().map(BtcToolkitMessage::MainView),
        Page::NetworkConfig => state
            .network_config
            .view()
            .map(BtcToolkitMessage::NetworkConfig),
        Page::DeviceDetail(_ip) => {
            if let Some(ref device_view) = state.device_detail_view {
                device_view.view().map(BtcToolkitMessage::DeviceDetail)
            } else {
                // Fallback to main view if no device detail available
                state.main_view.view().map(BtcToolkitMessage::MainView)
            }
        }
    }
}
