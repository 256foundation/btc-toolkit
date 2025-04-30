use crate::network::scanner::{ScanResult, ScanStatus, Scanner, ScannerMessage};
use crate::network_config::NetworkConfig;
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Element, Length};
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
    ScannerEvent(ScannerMessage),
    OpenIpInBrowser(Ipv4Addr),
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
                    self.scanning = true;

                    // Clear previous results
                    self.scanner.clear_results();

                    // Get IPs from network config
                    let ip_addresses = self.network_config.get_parsed_ips();

                    // Start the scan - this will update the scanner's internal state
                    // which will be displayed in the UI in real-time
                    self.scanner
                        .start_scan(ip_addresses)
                        .map(DashboardMessage::ScannerEvent)
                } else {
                    iced::Task::none()
                }
            }
            DashboardMessage::StopScan => {
                if self.scanning {
                    self.scanning = false;
                    self.scanner.stop_scan();
                }
                iced::Task::none()
            }
            DashboardMessage::ScannerEvent(scanner_msg) => {
                match scanner_msg {
                    ScannerMessage::ScanCompleted => {
                        self.scanning = false;
                    }
                    _ => {
                        // Other messages can be ignored since the scanner
                        // updates its internal state directly
                    }
                }
                iced::Task::none()
            }
            DashboardMessage::OpenIpInBrowser(ip) => {
                let url = format!("http://{}", ip);
                if let Err(e) = opener::open(&url) {
                    eprintln!("Failed to open URL {}: {}", url, e);
                    // Optionally, show an error message to the user in the UI
                }
                iced::Task::none()
            }
        }
    }

    pub fn set_network_config(&mut self, network_config: NetworkConfig) {
        self.network_config = network_config;
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

        let mut items = column![
            text("Scan Results").size(20),
            row![
                text("IP Address").width(Length::FillPortion(2)),
                text("Status").width(Length::FillPortion(2)),
                text("Miner Model").width(Length::FillPortion(3)),
            ]
            .spacing(10)
            .padding([0, 10])
        ]
        .spacing(10);

        // Sort results by IP address for consistent display
        let mut sorted_results: Vec<ScanResult> = results.values().cloned().collect();
        sorted_results.sort_by_key(|r| r.ip_address);

        for result in sorted_results {
            let status_text = match result.status {
                ScanStatus::Found => String::from("Found"),
                ScanStatus::NotFound => String::from("Not Found"),
                ScanStatus::Scanning => String::from("Scanning..."),
                ScanStatus::Pending => String::from("Pending"),
                ScanStatus::Error(err) => err,
            };

            let miner_model = match result.miner {
                Some(m) => m,
                None => String::from("-"),
            };

            let ip_button = button(text(result.ip_address.to_string()))
                .style(button::text)
                .width(Length::FillPortion(2))
                .on_press(DashboardMessage::OpenIpInBrowser(result.ip_address));

            items = items.push(
                row![
                    ip_button,
                    text(status_text).width(Length::FillPortion(2)),
                    text(miner_model).width(Length::FillPortion(3)),
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
