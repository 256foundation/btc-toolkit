mod config;
mod errors;
mod main_view;
mod network;
mod network_config;
mod sorting;
mod theme;
mod ui_helpers;

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
    iced::application("BTC Toolkit", update, view)
        .subscription(subscription)
        .window(window::Settings {
            size: Size::new(1200.0, 800.0),
            position: window::Position::Centered,
            min_size: Some(Size::new(1000.0, 650.0)),
            ..window::Settings::default()
        })
        .theme(|_| theme::THEME)
        .run_with(|| (BtcToolkit::new(), Task::none()))
}

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
    Scanner(ScannerMessage),
}

fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::MainView(message) => match message.clone() {
            MainViewMessage::OpenNetworkConfig | MainViewMessage::AddGroup => {
                state.current_page = Page::NetworkConfig;
                Task::none()
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
    match state.current_page {
        Page::Main => state.main_view.view().map(BtcToolkitMessage::MainView),
        Page::NetworkConfig => state
            .network_config
            .view()
            .map(BtcToolkitMessage::NetworkConfig),
    }
}
