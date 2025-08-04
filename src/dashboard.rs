use crate::network::scanner::{MinerInfo, Scanner};
use crate::network_config::NetworkConfig;
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Element, Length};
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
    OpenIpInBrowser(Ipv4Addr),
    NavigateToScanning(String, usize), // IP range and total count
}

// Main page state
pub struct Dashboard {
    scanning: bool,
    scanner: Scanner,
    network_config: NetworkConfig,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            scanning: false,
            scanner: Scanner::new(),
            network_config: NetworkConfig::new(),
        }
    }

    pub fn update(&mut self, message: DashboardMessage) -> iced::Task<DashboardMessage> {
        match message {
            DashboardMessage::OpenNetworkConfig => {
                // Leave empty - navigation is handled at the application level
                iced::Task::none()
            }
            DashboardMessage::StartScan => {
                if !self.scanning {
                    // Calculate total IPs to scan (simplified estimation)
                    let network_range = self.network_config.get_range();
                    let total_ips = self.estimate_ip_count(network_range);

                    // Create navigation message to scanning view
                    iced::Task::done(DashboardMessage::NavigateToScanning(
                        network_range.to_string(),
                        total_ips,
                    ))
                } else {
                    iced::Task::none()
                }
            }
            DashboardMessage::StopScan => {
                // Note: The new scanner doesn't support stopping mid-scan
                self.scanning = false;
                iced::Task::none()
            }

            DashboardMessage::OpenIpInBrowser(ip) => {
                let url = format!("http://{ip}");
                if let Err(e) = opener::open(&url) {
                    eprintln!("Failed to open URL {url}: {e}");
                    // Optionally, show an error message to the user in the UI
                }
                iced::Task::none()
            }
            DashboardMessage::NavigateToScanning(_, _) => {
                // This should be handled at the application level
                iced::Task::none()
            }
        }
    }

    pub fn set_network_config(&mut self, network_config: NetworkConfig) {
        self.network_config = network_config;
    }

    /// Set scan results from external source (e.g., scanning view)
    pub fn set_scan_results(&mut self, results: HashMap<Ipv4Addr, MinerInfo>) {
        self.scanner.set_results_from_map(results);
    }

    /// Estimate the number of IPs to scan based on the range
    fn estimate_ip_count(&self, range: &str) -> usize {
        if range.contains('/') {
            // CIDR notation, e.g., "192.168.1.0/24" = 254 IPs
            if let Some(prefix_len) = range.split('/').nth(1) {
                if let Ok(prefix) = prefix_len.parse::<u8>() {
                    let host_bits = 32 - prefix;
                    let total_ips = 2_usize.pow(host_bits as u32);
                    // Subtract network and broadcast addresses
                    return total_ips.saturating_sub(2).max(1);
                }
            }
        } else if range.contains('-') {
            // Range notation, e.g., "192.168.1.1-100" = 100 IPs
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                if let Ok(end) = parts[1].parse::<u8>() {
                    if let Some(start_part) = parts[0].split('.').next_back() {
                        if let Ok(start) = start_part.parse::<u8>() {
                            return (end.saturating_sub(start) + 1) as usize;
                        }
                    }
                }
            }
        }
        // Default fallback
        254
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

        // Create results view
        let results_view = self.view_scan_results();

        let content = column![
            title,
            subtitle,
            row![network_button, scan_button].spacing(10).padding(20),
            status_text,
            results_view
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

    fn view_scan_results(&self) -> Element<DashboardMessage> {
        let results = self.scanner.get_results();

        if results.is_empty() {
            return Space::new(Length::Fill, Length::Fixed(0.0)).into();
        }

        // Sort results by IP address for consistent display
        let mut sorted_miners: Vec<&MinerInfo> = results.values().collect();
        sorted_miners.sort_by_key(|m| m.ip);

        let mut items = column![
            text(format!("Found {} miners", sorted_miners.len())).size(20),
            row![
                text("IP Address").width(Length::FillPortion(2)),
                text("Model").width(Length::FillPortion(2)),
                text("Make").width(Length::FillPortion(2)),
                text("Firmware").width(Length::FillPortion(2)),
            ]
            .spacing(10)
            .padding([0, 10])
        ]
        .spacing(10);

        for miner in sorted_miners {
            let make_text = miner.make.as_deref().unwrap_or("Unknown");
            let firmware_text = miner.firmware.as_deref().unwrap_or("Unknown");

            let ip_button = button(text(miner.ip.to_string()))
                .style(button::text)
                .width(Length::FillPortion(2))
                .on_press(DashboardMessage::OpenIpInBrowser(miner.ip));

            items = items.push(
                row![
                    ip_button,
                    text(&miner.model).width(Length::FillPortion(2)),
                    text(make_text).width(Length::FillPortion(2)),
                    text(firmware_text).width(Length::FillPortion(2)),
                ]
                .spacing(10)
                .padding(5),
            );
        }

        scrollable(container(items).width(Length::Fill).padding(10))
            .height(Length::Fill)
            .into()
    }
}
