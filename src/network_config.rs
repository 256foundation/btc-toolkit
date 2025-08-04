use crate::network::scanner::ScanConfig;
use asic_rs::data::device::{MinerFirmware, MinerMake};
use iced::widget::{Space, button, checkbox, column, container, row, text, text_input};
use iced::{Element, Length};

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub ip_range: String,
    pub scan_config: ScanConfig,
    // UI state for checkboxes
    pub filter_antminers: bool,
    pub filter_whatsminers: bool,
    pub filter_avalonminers: bool,
    pub filter_braiins_os: bool,
    pub filter_stock_firmware: bool,
}

#[derive(Debug, Clone)]
pub enum NetworkConfigMessage {
    Close,
    SetIpRange(String),
    ToggleAntMiners(bool),
    ToggleWhatsMiners(bool),
    ToggleAvalonMiners(bool),
    ToggleBraiinsOS(bool),
    ToggleStockFirmware(bool),
    Save,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            ip_range: String::from("192.168.1.0/24"),
            scan_config: ScanConfig::default(),
            filter_antminers: false,
            filter_whatsminers: false,
            filter_avalonminers: false,
            filter_braiins_os: false,
            filter_stock_firmware: false,
        }
    }

    pub fn update(&mut self, msg: NetworkConfigMessage) {
        match msg {
            NetworkConfigMessage::SetIpRange(ip_range) => {
                self.ip_range = ip_range;
            }
            NetworkConfigMessage::ToggleAntMiners(enabled) => {
                self.filter_antminers = enabled;
                self.update_scan_config();
            }
            NetworkConfigMessage::ToggleWhatsMiners(enabled) => {
                self.filter_whatsminers = enabled;
                self.update_scan_config();
            }
            NetworkConfigMessage::ToggleAvalonMiners(enabled) => {
                self.filter_avalonminers = enabled;
                self.update_scan_config();
            }
            NetworkConfigMessage::ToggleBraiinsOS(enabled) => {
                self.filter_braiins_os = enabled;
                self.update_scan_config();
            }
            NetworkConfigMessage::ToggleStockFirmware(enabled) => {
                self.filter_stock_firmware = enabled;
                self.update_scan_config();
            }
            // We handle Open/Close events in the main app
            NetworkConfigMessage::Close | NetworkConfigMessage::Save => {}
        }
    }

    fn update_scan_config(&mut self) {
        let mut makes = Vec::new();
        let mut firmwares = Vec::new();

        // Build makes filter
        if self.filter_antminers {
            makes.push(MinerMake::AntMiner);
        }
        if self.filter_whatsminers {
            makes.push(MinerMake::WhatsMiner);
        }
        if self.filter_avalonminers {
            makes.push(MinerMake::AvalonMiner);
        }

        // Build firmware filter
        if self.filter_braiins_os {
            firmwares.push(MinerFirmware::BraiinsOS);
        }
        if self.filter_stock_firmware {
            firmwares.push(MinerFirmware::Stock);
        }

        // Update scan config
        self.scan_config = ScanConfig {
            search_makes: if makes.is_empty() { None } else { Some(makes) },
            search_firmwares: if firmwares.is_empty() {
                None
            } else {
                Some(firmwares)
            },
        };
    }

    pub fn get_range(&self) -> &str {
        &self.ip_range
    }

    pub fn get_scan_config(&self) -> &ScanConfig {
        &self.scan_config
    }

    pub fn view(&self) -> Element<NetworkConfigMessage> {
        let title = text("Network Configuration").size(24);

        let input_row = row![
            text("IP Range:").width(Length::Fixed(100.0)),
            text_input("e.g. 192.168.1.0/24 or 192.168.1.1-100", &self.ip_range)
                .on_input(NetworkConfigMessage::SetIpRange)
                .padding(8)
        ]
        .spacing(10);

        let help_text =
            text("Supports subnet notation (192.168.1.0/24) or ranges (192.168.1.1-100)").size(12);

        // Miner make filters
        let make_filters = column![
            text("Filter by Miner Make:").size(16),
            checkbox("AntMiner", self.filter_antminers)
                .on_toggle(NetworkConfigMessage::ToggleAntMiners),
            checkbox("WhatsMiner", self.filter_whatsminers)
                .on_toggle(NetworkConfigMessage::ToggleWhatsMiners),
            checkbox("AvalonMiner", self.filter_avalonminers)
                .on_toggle(NetworkConfigMessage::ToggleAvalonMiners),
        ]
        .spacing(8);

        // Firmware filters
        let firmware_filters = column![
            text("Filter by Firmware:").size(16),
            checkbox("Braiins OS", self.filter_braiins_os)
                .on_toggle(NetworkConfigMessage::ToggleBraiinsOS),
            checkbox("Stock Firmware", self.filter_stock_firmware)
                .on_toggle(NetworkConfigMessage::ToggleStockFirmware),
        ]
        .spacing(8);

        let filters_row = row![make_filters, firmware_filters].spacing(40);

        let filter_help = text("Leave all unchecked to scan for all miner types").size(10);

        let buttons = row![
            button(text("Cancel"))
                .padding(10)
                .on_press(NetworkConfigMessage::Close),
            button(text("Save"))
                .padding(10)
                .on_press(NetworkConfigMessage::Save)
        ]
        .spacing(10);

        let content = column![
            title,
            input_row,
            help_text,
            Space::new(Length::Fill, Length::Fixed(20.0)),
            filters_row,
            filter_help,
            Space::new(Length::Fill, Length::Fixed(20.0)),
            buttons
        ]
        .spacing(10)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
