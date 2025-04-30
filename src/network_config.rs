use crate::network::nmap_range;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length};
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub ip_range: String,
    ip_addresses: Vec<Ipv4Addr>,
}

#[derive(Debug, Clone)]
pub enum NetworkConfigMessage {
    Close,
    SetIpRange(String),
    Save,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            ip_range: String::from("192.1.1.1-50"),
            ip_addresses: Vec::new(),
        }
    }

    pub fn update(&mut self, msg: NetworkConfigMessage) {
        match msg {
            NetworkConfigMessage::SetIpRange(ip_range) => {
                self.ip_range = ip_range;
                self.ip_addresses = nmap_range::parse_nmap_range(&self.ip_range);
                println!("Set IP range to {}", self.ip_range);
                println!("IP addresses len: {:?}", self.ip_addresses.len());
            }
            // We handle Open/Close events in the main app
            NetworkConfigMessage::Close | NetworkConfigMessage::Save => {}
        }
    }

    pub fn get_parsed_ips(&self) -> Vec<Ipv4Addr> {
        if self.ip_addresses.is_empty() {
            nmap_range::parse_nmap_range(&self.ip_range)
        } else {
            self.ip_addresses.clone()
        }
    }

    pub fn view(&self) -> Element<NetworkConfigMessage> {
        let title = text("Network Configuration").size(24);

        let input_row = row![
            text("IP Range:").width(Length::Fixed(100.0)),
            text_input("e.g. 192.168.1.0/24", &self.ip_range)
                .on_input(NetworkConfigMessage::SetIpRange)
                .padding(8)
        ]
        .spacing(10);

        let ip_count = text(format!("IPs to scan: {}", self.ip_addresses.len())).size(14);

        let buttons = row![
            button(text("Cancel"))
                .padding(10)
                .on_press(NetworkConfigMessage::Close),
            button(text("Save"))
                .padding(10)
                .on_press(NetworkConfigMessage::Save)
        ]
        .spacing(10);

        let content = column![title, input_row, ip_count, buttons]
            .spacing(20)
            .padding(20)
            .max_width(400);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
