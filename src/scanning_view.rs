use crate::theme::{self, BtcTheme};
use asic_rs::data::miner::MinerData;
use iced::widget::{Space, button, column, container, progress_bar, row, scrollable, text};
use iced::{Element, Length};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum ScanningMessage {
    MinerFound {
        group_name: String,
        miner: MinerData,
    },
    GroupCompleted(String),
    GroupError {
        group_name: String,
        error: String,
    },
    AllScansCompleted,
    StopScan,
    BackToDashboard,
    OpenIpInBrowser(std::net::Ipv4Addr),
}

#[derive(Debug, Clone)]
pub struct ScanningView {
    discovered_miners_by_group: HashMap<String, Vec<MinerData>>,
    group_status: HashMap<String, GroupScanStatus>,
    total_groups: usize,
    completed_groups: usize,
    is_scanning: bool,
    start_time: Option<Instant>,
    total_ips_to_scan: usize,
    error_messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GroupScanStatus {
    pub completed: bool,
    pub error: Option<String>,
    pub miner_count: usize,
}

impl ScanningView {
    pub fn new(total_ips: usize) -> Self {
        Self::new_multi_group(1, total_ips)
    }

    pub fn new_multi_group(total_groups: usize, total_ips: usize) -> Self {
        Self {
            discovered_miners_by_group: HashMap::new(),
            group_status: HashMap::new(),
            total_groups,
            completed_groups: 0,
            is_scanning: true,
            start_time: Some(Instant::now()),
            total_ips_to_scan: total_ips,
            error_messages: Vec::new(),
        }
    }

    pub fn update(&mut self, message: ScanningMessage) {
        match message {
            ScanningMessage::MinerFound { group_name, miner } => {
                self.discovered_miners_by_group
                    .entry(group_name.clone())
                    .or_default()
                    .push(miner);

                // Update group status
                if let Some(status) = self.group_status.get_mut(&group_name) {
                    status.miner_count += 1;
                }
            }
            ScanningMessage::GroupCompleted(group_name) => {
                let miner_count = self
                    .discovered_miners_by_group
                    .get(&group_name)
                    .map(|miners| miners.len())
                    .unwrap_or(0);

                self.group_status.insert(
                    group_name,
                    GroupScanStatus {
                        completed: true,
                        error: None,
                        miner_count,
                    },
                );
                self.completed_groups += 1;
            }
            ScanningMessage::GroupError { group_name, error } => {
                self.group_status.insert(
                    group_name.clone(),
                    GroupScanStatus {
                        completed: true,
                        error: Some(error.clone()),
                        miner_count: self
                            .discovered_miners_by_group
                            .get(&group_name)
                            .map(|miners| miners.len())
                            .unwrap_or(0),
                    },
                );
                self.error_messages.push(format!("{group_name}: {error}"));
                self.completed_groups += 1;
            }
            ScanningMessage::AllScansCompleted => {
                self.is_scanning = false;
            }
            ScanningMessage::StopScan => {
                self.is_scanning = false;
            }
            ScanningMessage::BackToDashboard => {
                // This will be handled by the parent component
            }
            ScanningMessage::OpenIpInBrowser(ip) => {
                let url = format!("http://{ip}");
                if let Err(e) = opener::open(&url) {
                    eprintln!("Failed to open URL {url}: {e}");
                    // Optionally, show an error message to the user in the UI
                }
            }
        }
    }

    pub fn view(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        // Header with status
        let header = self.view_scan_header();

        // Real-time stats dashboard
        let stats_dashboard = self.view_stats_dashboard();

        // Progress and status section
        let progress_section = self.view_progress_section();

        // Main content in side-by-side layout
        let main_content = row![
            // Left panel: Group status and controls
            container(self.view_control_section())
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::FillPortion(1))
                .height(Length::Fill),
            // Right panel: Live results
            container(self.view_live_results())
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::FillPortion(2))
                .height(Length::Fill)
        ]
        .spacing(theme::layout::SPACING_MD)
        .height(Length::Fill);

        let content = column![header, stats_dashboard, progress_section, main_content]
            .spacing(theme::layout::SPACING_MD)
            .padding(theme::layout::PADDING_MD);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_scan_header(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        let status_icon = if self.is_scanning { "🔄" } else { "✅" };
        let status_text = if self.is_scanning {
            "Scanning in Progress"
        } else {
            "Scan Complete"
        };

        let header_row = row![
            column![
                theme::typography::title(format!("{status_icon} Network Scan")),
                theme::typography::small(status_text)
            ]
            .spacing(theme::layout::SPACING_XS),
            Space::new(Length::Fill, Length::Fixed(0.0)),
            // Live status indicator
            container(
                row![
                    text(status_icon).size(20),
                    column![
                        theme::typography::body(status_text),
                        theme::typography::tiny(if self.is_scanning {
                            "Please wait..."
                        } else {
                            "Ready for new scan"
                        })
                    ]
                    .spacing(theme::layout::SPACING_XS)
                ]
                .spacing(theme::layout::SPACING_SM)
                .align_y(iced::alignment::Vertical::Center)
            )
            .style(if self.is_scanning {
                theme::container_styles::status_warning
            } else {
                theme::container_styles::status_success
            })
            .padding([theme::layout::PADDING_SM, theme::layout::PADDING_MD])
        ]
        .align_y(iced::alignment::Vertical::Center);

        container(header_row)
            .style(theme::container_styles::header)
            .padding(theme::layout::PADDING_MD)
            .width(Length::Fill)
            .into()
    }

    fn view_stats_dashboard(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        let elapsed_time = self
            .start_time
            .map(|start| start.elapsed())
            .unwrap_or(Duration::ZERO);

        let total_miners: usize = self
            .discovered_miners_by_group
            .values()
            .map(|miners| miners.len())
            .sum();

        let stats = row![
            // Miners found
            container(
                column![
                    theme::typography::mono_large(total_miners.to_string()),
                    theme::typography::small("Miners Found"),
                    theme::typography::tiny("online devices")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // Groups progress
            container(
                column![
                    theme::typography::mono_large(format!(
                        "{}/{}",
                        self.completed_groups, self.total_groups
                    )),
                    theme::typography::small("Groups Complete"),
                    theme::typography::tiny("scan progress")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // Elapsed time
            container(
                column![
                    theme::typography::mono_large(format!("{:.1}s", elapsed_time.as_secs_f32())),
                    theme::typography::small("Elapsed Time"),
                    theme::typography::tiny("scanning duration")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // Error count
            container(
                column![
                    theme::typography::mono_large(self.error_messages.len().to_string()),
                    theme::typography::small("Errors"),
                    theme::typography::tiny("issues detected")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(move |theme| {
                if self.error_messages.is_empty() {
                    theme::container_styles::card(theme)
                } else {
                    theme::container_styles::status_error(theme)
                }
            })
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1))
        ]
        .spacing(theme::layout::SPACING_MD);

        stats.into()
    }

    fn view_progress_section(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        let overall_progress = if self.total_groups > 0 {
            self.completed_groups as f32 / self.total_groups as f32
        } else {
            0.0
        };

        let progress_bar_widget = progress_bar(0.0..=1.0, overall_progress)
            .style(theme::progress_bar_styles::scanning)
            .height(Length::Fixed(8.0));

        let progress_text = theme::typography::body(format!(
            "Overall Progress: {}% ({} of {} groups complete)",
            (overall_progress * 100.0) as u8,
            self.completed_groups,
            self.total_groups
        ));

        container(
            column![
                progress_text,
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),
                progress_bar_widget
            ]
            .spacing(theme::layout::SPACING_XS),
        )
        .style(iced::widget::container::rounded_box)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill)
        .into()
    }

    fn view_control_section(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        let section_header = theme::typography::heading("🎛️ Scan Control");

        // Control buttons
        let controls = if self.is_scanning {
            column![
                button(
                    row![
                        text("⏹️").size(16),
                        theme::typography::body("Stop All Scans")
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(theme::button_styles::danger)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::StopScan),
                theme::typography::small("⚠️ Stopping will cancel all active scans")
            ]
            .spacing(theme::layout::SPACING_SM)
        } else {
            column![
                button(
                    row![
                        text("🏠").size(16),
                        theme::typography::body("Back to Dashboard")
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(theme::button_styles::primary)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::BackToDashboard),
                button(
                    row![
                        text("🔄").size(16),
                        theme::typography::body("Start New Scan")
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(theme::button_styles::secondary)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::BackToDashboard)
            ]
            .spacing(theme::layout::SPACING_SM)
        };

        // Group status
        let group_status = self.view_group_status();

        // Error messages
        let error_section = self.view_error_section();

        column![
            section_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
            controls,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_LG)),
            group_status,
            error_section
        ]
        .spacing(theme::layout::SPACING_SM)
        .into()
    }

    fn view_live_results(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        let section_header = theme::typography::heading("⛏️ Miners");

        let results_content = self.view_discovered_miners();

        column![
            section_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
            results_content
        ]
        .spacing(theme::layout::SPACING_SM)
        .into()
    }

    fn view_error_section(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        if self.error_messages.is_empty() {
            return column![].into();
        }

        let mut error_list =
            column![theme::typography::heading("⚠️ Errors")].spacing(theme::layout::SPACING_SM);

        for error in &self.error_messages {
            let error_card = container(theme::typography::small(error))
                .style(theme::container_styles::status_error)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill);

            error_list = error_list.push(error_card);
        }

        container(error_list)
            .padding([theme::layout::PADDING_LG, 0.0])
            .into()
    }

    fn view_group_status(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        if self.group_status.is_empty() && self.discovered_miners_by_group.is_empty() {
            return container(
                column![
                    text("🔍").size(24),
                    theme::typography::body("Preparing scans..."),
                    theme::typography::small("Initializing network discovery")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_SM),
            )
            .padding(theme::layout::PADDING_MD)
            .into();
        }

        let header = theme::typography::heading("📊 Group Status");
        let mut status_list = column![].spacing(theme::layout::SPACING_SM);

        // Show status for groups that have started
        for (group_name, miners) in &self.discovered_miners_by_group {
            let status = self.group_status.get(group_name);
            let (status_icon, status_text) = match status {
                Some(s) if s.completed && s.error.is_some() => ("❌", "Error"),
                Some(s) if s.completed => ("✅", "Complete"),
                _ => ("🔄", "Scanning"),
            };

            let group_card = container(
                row![
                    column![
                        row![
                            text(status_icon).size(16),
                            theme::typography::body(group_name)
                        ]
                        .spacing(theme::layout::SPACING_XS)
                        .align_y(iced::alignment::Vertical::Center),
                        theme::typography::small(format!("{} miners found", miners.len()))
                    ]
                    .spacing(theme::layout::SPACING_XS)
                    .width(Length::Fill),
                    {
                        let status_container = container(theme::typography::small(status_text))
                            .padding([theme::layout::PADDING_XS, theme::layout::PADDING_SM]);

                        match status {
                            Some(s) if s.completed && s.error.is_some() => {
                                status_container.style(theme::container_styles::status_error)
                            }
                            Some(s) if s.completed => {
                                status_container.style(theme::container_styles::status_success)
                            }
                            _ => status_container.style(theme::container_styles::status_warning),
                        }
                    }
                ]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(theme::layout::SPACING_SM),
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill);

            status_list = status_list.push(group_card);
        }

        column![
            header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),
            scrollable(status_list).height(Length::Fixed(150.0))
        ]
        .spacing(theme::layout::SPACING_XS)
        .into()
    }

    fn view_discovered_miners(&self) -> Element<ScanningMessage> {
        let _theme = BtcTheme::default();

        if self.discovered_miners_by_group.is_empty() {
            return container(
                column![
                    text("🔍").size(32),
                    theme::typography::body("Scanning for miners..."),
                    theme::typography::small(
                        "Live results will appear here as devices are discovered"
                    )
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_SM),
            )
            .padding(theme::layout::PADDING_LG)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let total_miners: usize = self
            .discovered_miners_by_group
            .values()
            .map(|m| m.len())
            .sum();

        let mut results_content = column![].spacing(theme::layout::SPACING_MD);

        for (group_name, miners) in self.discovered_miners_by_group.iter() {
            if miners.is_empty() {
                continue;
            }

            // Group header with live update indicator
            let group_header = container(
                row![
                    column![
                        theme::typography::heading(format!("⛏️ {group_name}")),
                        theme::typography::small(format!("{} miners discovered", miners.len()))
                    ]
                    .spacing(theme::layout::SPACING_XS),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                    if self.is_scanning {
                        container(
                            row![text("🔄").size(14), theme::typography::tiny("Live")]
                                .spacing(theme::layout::SPACING_XS)
                                .align_y(iced::alignment::Vertical::Center),
                        )
                        .style(theme::container_styles::status_warning)
                        .padding([theme::layout::PADDING_XS, theme::layout::PADDING_SM])
                    } else {
                        container(text(""))
                    }
                ]
                .align_y(iced::alignment::Vertical::Center),
            )
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill);

            // Table header
            let table_header = container(
                row![
                    theme::typography::small("IP Address").width(Length::FillPortion(3)),
                    theme::typography::small("Model").width(Length::FillPortion(3)),
                    theme::typography::small("Make").width(Length::FillPortion(2)),
                    theme::typography::small("Firmware").width(Length::FillPortion(2)),
                ]
                .spacing(theme::layout::SPACING_SM),
            )
            .style(theme::container_styles::header)
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill);

            let mut miners_list = column![].spacing(theme::layout::SPACING_XS);

            // Sort miners by IP for consistent display
            let mut sorted_miners = miners.clone();
            sorted_miners.sort_by_key(|m| m.ip);

            for miner in sorted_miners {
                let miner_ip = match miner.ip {
                    std::net::IpAddr::V4(ipv4) => ipv4,
                    std::net::IpAddr::V6(_) => continue, // Skip IPv6 addresses for now
                };

                let miner_row = container(
                    row![
                        container(
                            row![
                                text("🟢").size(12),
                                button(theme::typography::mono(miner_ip.to_string()))
                                    .style(iced::widget::button::text)
                                    .padding(theme::layout::PADDING_XS)
                                    .on_press(ScanningMessage::OpenIpInBrowser(miner_ip))
                            ]
                            .spacing(theme::layout::SPACING_XS)
                            .align_y(iced::alignment::Vertical::Center)
                        )
                        .width(Length::FillPortion(3)),
                        theme::typography::body(&format!("{:?}", miner.device_info.model))
                            .width(Length::FillPortion(3)),
                        theme::typography::body(&format!("{:?}", miner.device_info.make))
                            .width(Length::FillPortion(2)),
                        theme::typography::body(&format!("{:?}", miner.device_info.firmware))
                            .width(Length::FillPortion(2)),
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center),
                )
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill);

                miners_list = miners_list.push(miner_row);
            }

            let group_section = column![
                group_header,
                table_header,
                scrollable(miners_list).height(Length::Shrink)
            ]
            .spacing(theme::layout::SPACING_XS);

            results_content = results_content.push(group_section);
        }

        // Summary at the top if multiple groups
        if self.discovered_miners_by_group.len() > 1 {
            let summary = container(theme::typography::heading(format!(
                "📈 {total_miners} Total Miners Discovered"
            )))
            .style(theme::container_styles::status_success)
            .padding(theme::layout::PADDING_MD)
            .width(Length::Fill);

            column![summary, scrollable(results_content).height(Length::Fill)]
                .spacing(theme::layout::SPACING_MD)
                .into()
        } else {
            scrollable(results_content).height(Length::Fill).into()
        }
    }

    pub fn get_discovered_miners_by_group(&self) -> HashMap<String, Vec<MinerData>> {
        self.discovered_miners_by_group.clone()
    }
}
