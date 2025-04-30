use iced::widget::{button, container, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
}

// Main page state
pub struct Dashboard {
    scanning: bool,
}

impl Dashboard {
    pub fn new() -> Self {
        Self { scanning: false }
    }

    pub fn update(&mut self, message: DashboardMessage) {
        match message {
            DashboardMessage::OpenNetworkConfig => {
                // Leave empty - navigation is handled at the application level
                // We explicitly leave this empty to show that the message is recognized
                // but handled at a higher level in the component hierarchy
            }
            DashboardMessage::StartScan => {
                self.scanning = true;
            }
            DashboardMessage::StopScan => {
                self.scanning = false;
            }
        }
    }

    pub fn view(&self) -> Element<DashboardMessage> {
        let title = text("BTC ASIC Miner Scanner").size(28);
        let subtitle = text("A tool for scanning local network for Bitcoin ASIC miners").size(16);

        let network_button = button(text("Configure Network"))
            .padding(10)
            .on_press(DashboardMessage::OpenNetworkConfig);

        let scan_button = if self.scanning {
            button(text("Stop Scan"))
                .padding(10)
                .on_press(DashboardMessage::StopScan)
        } else {
            button(text("Start Scan"))
                .padding(10)
                .on_press(DashboardMessage::StartScan)
        };

        let status_text = if self.scanning {
            text("Scanning network for ASIC miners...").size(14)
        } else {
            text("Ready to scan").size(14)
        };

        let content = iced::widget::column![
            title,
            subtitle,
            row![network_button, scan_button].spacing(10).padding(20),
            status_text
        ]
        .spacing(20)
        .align_x(iced::alignment::Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
