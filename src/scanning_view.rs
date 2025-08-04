use crate::network::scanner::MinerInfo;
use iced::widget::{Space, button, column, container, progress_bar, row, scrollable, text};
use iced::{Element, Length};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum ScanningMessage {
    MinerFound(MinerInfo),
    ScanCompleted,
    ScanError(String),
    StopScan,
    BackToDashboard,
}

#[derive(Debug, Clone)]
pub struct ScanningView {
    discovered_miners: HashMap<Ipv4Addr, MinerInfo>,
    scan_progress: f32,
    is_scanning: bool,
    start_time: Option<Instant>,
    total_ips_to_scan: usize,
    scanned_ips: usize,
    error_message: Option<String>,
}

impl ScanningView {
    pub fn new(total_ips: usize) -> Self {
        Self {
            discovered_miners: HashMap::new(),
            scan_progress: 0.0,
            is_scanning: true,
            start_time: Some(Instant::now()),
            total_ips_to_scan: total_ips,
            scanned_ips: 0,
            error_message: None,
        }
    }

    pub fn update(&mut self, message: ScanningMessage) {
        match message {
            ScanningMessage::MinerFound(miner) => {
                self.discovered_miners.insert(miner.ip, miner);
                // Update progress based on discovery
                self.scanned_ips += 1;
                self.scan_progress = if self.total_ips_to_scan > 0 {
                    self.scanned_ips as f32 / self.total_ips_to_scan as f32
                } else {
                    1.0
                };
            }
            ScanningMessage::ScanCompleted => {
                self.is_scanning = false;
                self.scan_progress = 1.0;
                self.scanned_ips = self.total_ips_to_scan;
            }
            ScanningMessage::ScanError(error) => {
                self.is_scanning = false;
                self.error_message = Some(error);
            }
            ScanningMessage::StopScan => {
                self.is_scanning = false;
            }
            ScanningMessage::BackToDashboard => {
                // This will be handled by the parent component
            }
        }
    }

    pub fn view(&self) -> Element<ScanningMessage> {
        let title = text("Network Scan in Progress")
            .size(24)
            .width(Length::Fill);

        // Scan statistics
        let elapsed_time = self
            .start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::ZERO);

        let stats_row = row![
            text(format!("Found: {}", self.discovered_miners.len())).size(16),
            text(format!(
                "Scanned: {}/{}",
                self.scanned_ips, self.total_ips_to_scan
            ))
            .size(16),
            text(format!("Time: {:.1}s", elapsed_time.as_secs_f32())).size(16),
        ]
        .spacing(20);

        // Progress bar
        let progress = progress_bar(0.0..=1.0, self.scan_progress);

        // Control buttons
        let controls = if self.is_scanning {
            row![
                button(text("Stop Scan"))
                    .padding(10)
                    .on_press(ScanningMessage::StopScan)
            ]
        } else {
            row![
                button(text("Back to Dashboard"))
                    .padding(10)
                    .on_press(ScanningMessage::BackToDashboard),
                Space::new(Length::Fixed(10.0), Length::Fixed(0.0)),
                if self.error_message.is_some() {
                    button(text("Retry"))
                        .padding(10)
                        .on_press(ScanningMessage::BackToDashboard) // For now, go back to retry
                } else {
                    button(text("New Scan"))
                        .padding(10)
                        .on_press(ScanningMessage::BackToDashboard)
                }
            ]
        };

        // Error message if any
        let error_section = if let Some(ref error) = self.error_message {
            column![
                Space::new(Length::Fixed(0.0), Length::Fixed(10.0)),
                text(format!("Error: {error}")).size(14)
            ]
        } else {
            column![]
        };

        // Discovered miners list
        let miners_section = if self.discovered_miners.is_empty() {
            column![
                Space::new(Length::Fixed(0.0), Length::Fixed(20.0)),
                text("No miners discovered yet...").size(14)
            ]
        } else {
            let mut miners_list = column![
                Space::new(Length::Fixed(0.0), Length::Fixed(20.0)),
                text(format!(
                    "Discovered Miners ({}):",
                    self.discovered_miners.len()
                ))
                .size(18),
                // Header row
                row![
                    text("IP Address").width(Length::FillPortion(3)),
                    text("Model").width(Length::FillPortion(3)),
                    text("Make").width(Length::FillPortion(2)),
                    text("Firmware").width(Length::FillPortion(2)),
                ]
                .spacing(10)
                .padding([5, 10])
            ]
            .spacing(5);

            // Sort miners by IP for consistent display
            let mut sorted_miners: Vec<&MinerInfo> = self.discovered_miners.values().collect();
            sorted_miners.sort_by_key(|m| m.ip);

            for miner in sorted_miners {
                let miner_row = row![
                    text(miner.ip.to_string()).width(Length::FillPortion(3)),
                    text(&miner.model).width(Length::FillPortion(3)),
                    text(miner.make.as_deref().unwrap_or("Unknown")).width(Length::FillPortion(2)),
                    text(miner.firmware.as_deref().unwrap_or("Unknown"))
                        .width(Length::FillPortion(2)),
                ]
                .spacing(10)
                .padding([5, 10]);

                miners_list = miners_list.push(container(miner_row).width(Length::Fill));
            }

            miners_list
        };

        let content = column![
            title,
            Space::new(Length::Fixed(0.0), Length::Fixed(20.0)),
            stats_row,
            Space::new(Length::Fixed(0.0), Length::Fixed(10.0)),
            progress,
            Space::new(Length::Fixed(0.0), Length::Fixed(10.0)),
            controls,
            error_section,
        ]
        .spacing(10)
        .padding(20);

        let scrollable_content = scrollable(column![content, miners_section]);

        container(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn get_discovered_miners(&self) -> &HashMap<Ipv4Addr, MinerInfo> {
        &self.discovered_miners
    }
}
