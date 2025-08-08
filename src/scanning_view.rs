use crate::theme;
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
    SortColumn(SortColumn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    IpAddress,
    Model,
    Make,
    Firmware,
    FirmwareVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
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
    sort_column: Option<SortColumn>,
    sort_direction: SortDirection,
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
            sort_column: Some(SortColumn::IpAddress),
            sort_direction: SortDirection::Ascending,
        }
    }

    //noinspection HttpUrlsUsage
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
            ScanningMessage::SortColumn(column) => {
                if Some(column) == self.sort_column {
                    // Toggle direction if clicking the same column
                    self.sort_direction = match self.sort_direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };
                } else {
                    // New column, default to ascending
                    self.sort_column = Some(column);
                    self.sort_direction = SortDirection::Ascending;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, ScanningMessage> {
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
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1))
                .height(Length::Fill),
            // Right panel: Live results
            container(self.view_live_results())
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(2))
                .height(Length::Fill)
        ]
        .spacing(theme::spacing::MD)
        .height(Length::Fill);

        let content = column![
            row![column![header]],
            row![
                column![stats_dashboard, progress_section, main_content]
                    .spacing(theme::spacing::MD)
                    .padding(theme::padding::MD)
            ]
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_scan_header(&self) -> Element<'_, ScanningMessage> {
        let status_text = if self.is_scanning {
            "Scanning in Progress"
        } else {
            "Scan Complete"
        };

        let header_row = row![
            column![
                theme::typography::title("Network Scan"),
                theme::typography::small(status_text)
            ]
            .spacing(theme::spacing::XS),
            Space::new(Length::Fill, Length::Fixed(0.0)),
            // Live status indicator
            container(
                row![
                    column![
                        theme::typography::body(status_text),
                        theme::typography::tiny(if self.is_scanning {
                            "Please wait..."
                        } else {
                            "Ready for new scan"
                        })
                    ]
                    .spacing(theme::spacing::XS)
                ]
                .spacing(theme::spacing::SM)
                .align_y(iced::alignment::Vertical::Center)
            )
            .style(if self.is_scanning {
                theme::containers::warning
            } else {
                theme::containers::success
            })
            .padding([theme::padding::SM, theme::padding::MD])
        ]
        .align_y(iced::alignment::Vertical::Center);

        container(header_row)
            .style(theme::containers::header)
            .padding(theme::padding::MD)
            .width(Length::Fill)
            .into()
    }

    fn view_stats_dashboard(&self) -> Element<'_, ScanningMessage> {
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
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
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
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::FillPortion(1)),
            // Elapsed time
            container(
                column![
                    theme::typography::mono_large(format!("{:.1}s", elapsed_time.as_secs_f32())),
                    theme::typography::small("Elapsed Time"),
                    theme::typography::tiny("scanning duration")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::FillPortion(1)),
            // Error count
            container(
                column![
                    theme::typography::mono_large(self.error_messages.len().to_string()),
                    theme::typography::small("Errors"),
                    theme::typography::tiny("issues detected")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS)
            )
            .style(if self.error_messages.is_empty() {
                theme::containers::card
            } else {
                theme::containers::error
            })
            .align_x(iced::alignment::Horizontal::Center)
            .padding(theme::padding::MD)
            .width(Length::FillPortion(1))
        ]
        .spacing(theme::spacing::MD);

        stats.into()
    }

    fn view_progress_section(&self) -> Element<'_, ScanningMessage> {
        let overall_progress = if self.total_groups > 0 {
            self.completed_groups as f32 / self.total_groups as f32
        } else {
            0.0
        };

        let progress_bar_widget = progress_bar(0.0..=1.0, overall_progress)
            // .style(theme::progress_bar_styles::scanning)
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
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::SM)),
                progress_bar_widget
            ]
            .spacing(theme::spacing::XS),
        )
        .style(theme::containers::card)
        .padding(theme::padding::MD)
        .width(Length::Fill)
        .into()
    }

    fn view_control_section(&self) -> Element<'_, ScanningMessage> {
        let panel_header = row![
            theme::typography::heading("Scan Control"),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ];

        // Control buttons
        let controls = if self.is_scanning {
            column![
                button(
                    row![
                        theme::typography::body("Stop All Scans")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::danger)
                .padding(theme::padding::SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::StopScan),
            ]
            .spacing(theme::spacing::MD)
        } else {
            column![
                button(
                    row![
                        theme::typography::body("Back To Dashboard")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::primary)
                .padding(theme::padding::SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::BackToDashboard),
                button(
                    row![
                        theme::typography::body("Start New Scan")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::secondary)
                .padding(theme::padding::SM)
                .width(Length::Fill)
                .on_press(ScanningMessage::BackToDashboard)
            ]
            .spacing(theme::spacing::MD)
        };

        // Group status
        let group_status = self.view_group_status();

        // Error messages
        let error_section = self.view_error_section();

        column![
            panel_header,
            controls,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::MD)),
            group_status,
            error_section
        ]
        .spacing(theme::spacing::SM)
        .into()
    }

    fn view_live_results(&self) -> Element<'_, ScanningMessage> {
        let section_header = theme::typography::heading("Miners");

        let results_content = self.view_discovered_miners();

        column![
            section_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::MD)),
            results_content
        ]
        .spacing(theme::spacing::SM)
        .into()
    }

    fn view_error_section(&self) -> Element<'_, ScanningMessage> {
        if self.error_messages.is_empty() {
            return column![].into();
        }

        let mut error_list =
            column![theme::typography::heading("Errors")].spacing(theme::spacing::SM);

        for error in &self.error_messages {
            let error_card = container(theme::typography::small(error))
                .style(theme::containers::error)
                .padding(theme::padding::SM)
                .width(Length::Fill);

            error_list = error_list.push(error_card);
        }

        container(error_list)
            .padding([theme::padding::LG, 0.0])
            .into()
    }

    fn view_group_status(&self) -> Element<'_, ScanningMessage> {
        if self.group_status.is_empty() && self.discovered_miners_by_group.is_empty() {
            return container(
                column![
                    theme::typography::body("Preparing scans..."),
                    theme::typography::small("Initializing network discovery")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .width(Length::Fill)
                .spacing(theme::spacing::SM),
            )
            .align_y(iced::alignment::Vertical::Center)
            .height(Length::Fill)
            .padding(theme::padding::MD)
            .into();
        }

        let header = theme::typography::heading("Group Status");
        let mut status_list = column![].spacing(theme::spacing::SM);

        // Show status for groups that have started
        for (group_name, miners) in &self.discovered_miners_by_group {
            let status = self.group_status.get(group_name);
            let status_text = match status {
                Some(s) if s.completed && s.error.is_some() => "Error",
                Some(s) if s.completed => "Complete",
                _ => "Scanning",
            };

            let group_card = container(
                row![
                    column![
                        row![theme::typography::body(group_name)]
                            .spacing(theme::spacing::XS)
                            .align_y(iced::alignment::Vertical::Center),
                        theme::typography::small(format!("{} miners found", miners.len()))
                    ]
                    .spacing(theme::spacing::XS)
                    .width(Length::Fill),
                    {
                        let status_container = container(theme::typography::small(status_text))
                            .padding([theme::padding::XS, theme::padding::SM]);

                        match status {
                            Some(s) if s.completed && s.error.is_some() => {
                                status_container.style(theme::containers::error)
                            }
                            Some(s) if s.completed => {
                                status_container.style(theme::containers::success)
                            }
                            _ => status_container.style(theme::containers::warning),
                        }
                    }
                ]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(theme::spacing::SM),
            )
            .style(theme::containers::card)
            .padding(theme::padding::SM)
            .width(Length::Fill);

            status_list = status_list.push(group_card);
        }

        column![
            header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::SM)),
            scrollable(status_list).height(Length::Fixed(150.0))
        ]
        .spacing(theme::spacing::XS)
        .into()
    }

    fn view_discovered_miners(&self) -> Element<'_, ScanningMessage> {
        if self.discovered_miners_by_group.is_empty() {
            return container(
                column![
                    theme::typography::body("Scanning for miners..."),
                    theme::typography::small(
                        "Live results will appear here as devices are discovered"
                    )
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::SM),
            )
            .padding(theme::padding::LG)
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

        let mut results_content = column![].spacing(theme::spacing::MD);

        for (group_name, miners) in self.discovered_miners_by_group.iter() {
            if miners.is_empty() {
                continue;
            }

            // Group header with live update indicator
            let group_header = container(
                row![
                    column![
                        theme::typography::heading(format!("{group_name}")),
                        theme::typography::small(format!("{} miners discovered", miners.len()))
                    ]
                    .spacing(theme::spacing::XS),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                    if self.is_scanning {
                        container(
                            row![theme::typography::tiny("Live")]
                                .spacing(theme::spacing::XS)
                                .align_y(iced::alignment::Vertical::Center),
                        )
                        .style(theme::containers::warning)
                        .padding([theme::padding::XS, theme::padding::SM])
                    } else {
                        container(text(""))
                    }
                ]
                .align_y(iced::alignment::Vertical::Center),
            )
            .padding(theme::padding::SM)
            .width(Length::Fill);

            // Table header with sortable columns
            let table_header = container(
                row![
                    container(
                        button(
                            row![
                                theme::typography::small("IP Address"),
                                self.get_sort_indicator(SortColumn::IpAddress)
                            ]
                            .align_y(iced::alignment::Vertical::Center)
                            .spacing(theme::spacing::XS)
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(ScanningMessage::SortColumn(SortColumn::IpAddress))
                    )
                    .width(Length::FillPortion(3))
                    .padding(theme::padding::XS),
                    container(
                        button(
                            row![
                                theme::typography::small("Model"),
                                self.get_sort_indicator(SortColumn::Model)
                            ]
                            .align_y(iced::alignment::Vertical::Center)
                            .spacing(theme::spacing::XS)
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(ScanningMessage::SortColumn(SortColumn::Model))
                    )
                    .width(Length::FillPortion(3))
                    .padding(theme::padding::XS),
                    container(
                        button(
                            row![
                                theme::typography::small("Make"),
                                self.get_sort_indicator(SortColumn::Make)
                            ]
                            .align_y(iced::alignment::Vertical::Center)
                            .spacing(theme::spacing::XS)
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(ScanningMessage::SortColumn(SortColumn::Make))
                    )
                    .width(Length::FillPortion(2))
                    .padding(theme::padding::XS),
                    container(
                        button(
                            row![
                                theme::typography::small("Firmware"),
                                self.get_sort_indicator(SortColumn::Firmware)
                            ]
                            .align_y(iced::alignment::Vertical::Center)
                            .spacing(theme::spacing::XS)
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(ScanningMessage::SortColumn(SortColumn::Firmware))
                    )
                    .width(Length::FillPortion(2))
                    .padding(theme::padding::XS),
                    container(
                        button(
                            row![
                                theme::typography::small("Firmware Version"),
                                self.get_sort_indicator(SortColumn::FirmwareVersion)
                            ]
                            .align_y(iced::alignment::Vertical::Center)
                            .spacing(theme::spacing::XS)
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(ScanningMessage::SortColumn(SortColumn::FirmwareVersion))
                    )
                    .width(Length::FillPortion(2))
                    .padding(theme::padding::XS),
                ]
                .spacing(theme::spacing::SM),
            )
            .style(theme::containers::header)
            .padding(theme::padding::SM)
            .width(Length::Fill);

            let mut miners_list = column![]
                .spacing(theme::spacing::XS)
                .padding(theme::padding::SCROLLABLE);

            // Sort miners based on selected column
            let mut sorted_miners = miners.clone();
            self.sort_miners(&mut sorted_miners);

            for miner in sorted_miners {
                let miner_ip = match miner.ip {
                    std::net::IpAddr::V4(ipv4) => ipv4,
                    std::net::IpAddr::V6(_) => continue, // Skip IPv6 addresses for now
                };

                let miner_row = container(
                    row![
                        container(
                            button(theme::typography::mono(miner_ip.to_string()))
                                .style(button::text)
                                .padding(0)
                                .on_press(ScanningMessage::OpenIpInBrowser(miner_ip))
                        )
                        .width(Length::FillPortion(3))
                        .padding(theme::padding::XS),
                        container(theme::typography::body(
                            format!("{}", miner.device_info.model).replace("Plus", "+")
                        ))
                        .width(Length::FillPortion(3))
                        .padding(theme::padding::XS),
                        container(theme::typography::body(format!(
                            "{}",
                            miner.device_info.make
                        )))
                        .width(Length::FillPortion(2))
                        .padding(theme::padding::XS),
                        container(theme::typography::body(format!(
                            "{}",
                            miner.device_info.firmware
                        )))
                        .width(Length::FillPortion(2))
                        .padding(theme::padding::XS),
                        container(theme::typography::body(format!(
                            "{}",
                            miner.firmware_version.unwrap_or("-".into())
                        )))
                        .width(Length::FillPortion(2))
                        .padding(theme::padding::XS),
                    ]
                    .spacing(theme::spacing::SM)
                    .align_y(iced::alignment::Vertical::Center),
                )
                .style(theme::containers::card)
                .padding(theme::padding::SM)
                .width(Length::Fill);

                miners_list = miners_list.push(miner_row);
            }

            let group_section = column![
                group_header,
                table_header.padding(theme::padding::SCROLLABLE),
                scrollable(miners_list)
            ]
            .spacing(theme::spacing::XS);

            results_content = results_content.push(group_section);
        }

        // Summary at the top if multiple groups
        if self.discovered_miners_by_group.len() > 1 {
            let summary = container(theme::typography::heading(format!(
                "{total_miners} Total Miners Discovered"
            )))
            .style(theme::containers::success)
            .padding(theme::padding::MD)
            .width(Length::Fill);

            column![summary, results_content]
                .spacing(theme::spacing::MD)
                .into()
        } else {
            results_content.into()
        }
    }

    pub fn get_discovered_miners_by_group(&self) -> HashMap<String, Vec<MinerData>> {
        self.discovered_miners_by_group.clone()
    }

    fn get_sort_indicator(&self, column: SortColumn) -> Element<'_, ScanningMessage> {
        if Some(column) == self.sort_column {
            let arrow = match self.sort_direction {
                SortDirection::Ascending => "▲",
                SortDirection::Descending => "▼",
            };
            theme::typography::small(arrow).into()
        } else {
            text("").into()
        }
    }

    fn sort_miners(&self, miners: &mut Vec<MinerData>) {
        match self.sort_column {
            Some(SortColumn::IpAddress) => {
                miners.sort_by(|a, b| {
                    let result = a.ip.cmp(&b.ip);
                    if self.sort_direction == SortDirection::Descending {
                        result.reverse()
                    } else {
                        result
                    }
                });
            }
            Some(SortColumn::Model) => {
                miners.sort_by(|a, b| {
                    let result = a
                        .device_info
                        .model
                        .to_string()
                        .cmp(&b.device_info.model.to_string());
                    if self.sort_direction == SortDirection::Descending {
                        result.reverse()
                    } else {
                        result
                    }
                });
            }
            Some(SortColumn::Make) => {
                miners.sort_by(|a, b| {
                    let result = a
                        .device_info
                        .make
                        .to_string()
                        .cmp(&b.device_info.make.to_string());
                    if self.sort_direction == SortDirection::Descending {
                        result.reverse()
                    } else {
                        result
                    }
                });
            }
            Some(SortColumn::Firmware) => {
                miners.sort_by(|a, b| {
                    let result = a
                        .device_info
                        .firmware
                        .to_string()
                        .cmp(&b.device_info.firmware.to_string());
                    if self.sort_direction == SortDirection::Descending {
                        result.reverse()
                    } else {
                        result
                    }
                });
            }
            Some(SortColumn::FirmwareVersion) => {
                miners.sort_by(|a, b| {
                    let a_version = a.firmware_version.as_deref().unwrap_or("");
                    let b_version = b.firmware_version.as_deref().unwrap_or("");
                    let result = a_version.cmp(b_version);
                    if self.sort_direction == SortDirection::Descending {
                        result.reverse()
                    } else {
                        result
                    }
                });
            }
            None => {
                // Should not happen now as we have a default, but fallback to IP address
                miners.sort_by_key(|m| m.ip);
            }
        }
    }
}
