use iced::{
    Length, Size,
    widget::{self, column, text},
    window,
};
use mimalloc::MiMalloc;

// github.com/microsoft/mimalloc
// https://github.com/purpleprotocol/mimalloc_rust
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> iced::Result {
    // Create application with title, update function, and view function
    iced::application("BTC ASIC Miner Scanner", update, view)
        // Configure window with custom settings
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        // Run with initial state
        .run_with(|| (BtcScannerApp::new(), iced::Task::none()))
}

struct BtcScannerApp;

impl BtcScannerApp {
    fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
enum Message {}

// Update function for the application
fn update(_state: &mut BtcScannerApp, _message: Message) -> iced::Task<Message> {
    iced::Task::none()
}

// View function for the application
fn view(_state: &BtcScannerApp) -> iced::Element<Message> {
    let content = column![
        text("BTC ASIC Miner Scanner").size(28),
        text("A tool for scanning local network for Bitcoin ASIC miners").size(16)
    ]
    .spacing(20)
    .align_x(iced::alignment::Horizontal::Center);

    widget::container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
