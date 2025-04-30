mod dashboard;
mod network;
mod network_config;

use crate::dashboard::{Dashboard, DashboardMessage};
use crate::network_config::{NetworkConfig, NetworkConfigMessage};
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
}

struct BtcToolkit {
    current_page: Page,
    main_page: Dashboard,
    network_config: NetworkConfig,
}

impl BtcToolkit {
    fn new() -> Self {
        Self {
            current_page: Page::Dashboard,
            main_page: Dashboard::new(),
            network_config: NetworkConfig::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum BtcToolkitMessage {
    ChangePage(Page),
    Dashboard(DashboardMessage),
    NetworkConfig(NetworkConfigMessage),
}

// Update function for the application
fn update(state: &mut BtcToolkit, message: BtcToolkitMessage) -> iced::Task<BtcToolkitMessage> {
    match message {
        BtcToolkitMessage::ChangePage(page) => {
            state.current_page = page;
            iced::Task::none()
        }
        BtcToolkitMessage::Dashboard(message) => {
            let task = state.main_page.update(message.clone());

            match message {
                DashboardMessage::OpenNetworkConfig => {
                    state.current_page = Page::NetworkConfig;
                    iced::Task::none()
                }
                _ => task.map(BtcToolkitMessage::Dashboard),
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
                _ => iced::Task::none(),
            }
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
    }
}
