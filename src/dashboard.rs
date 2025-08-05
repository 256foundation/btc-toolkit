use crate::config::AppConfig;
use crate::theme::{self, BtcTheme};
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Element, Length};
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub enum DashboardMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
    OpenIpInBrowser(Ipv4Addr),
    NavigateToScanning,
}

// Main page state
pub struct Dashboard {
    scanning: bool,
    app_config: AppConfig,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            scanning: false,
            app_config: AppConfig::default(),
        }
    }

    pub fn set_app_config(&mut self, config: AppConfig) {
        self.app_config = config;
    }

    pub fn update(&mut self, message: DashboardMessage) -> iced::Task<DashboardMessage> {
        match message {
            DashboardMessage::OpenNetworkConfig => {
                // Leave empty - navigation is handled at the application level
                iced::Task::none()
            }
            DashboardMessage::StartScan => {
                if !self.scanning {
                    // Navigate to scanning view (logic handled in main app)
                    iced::Task::done(DashboardMessage::NavigateToScanning)
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
            DashboardMessage::NavigateToScanning => {
                // This should be handled at the application level
                iced::Task::none()
            }
        }
    }

    /// Estimate the number of IPs to scan based on the range
    pub fn estimate_ip_count(&self, range: &str) -> usize {
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
        let _theme = BtcTheme::default();

        // Header with title and system status
        let header = self.view_header();

        // Quick stats cards
        let stats_cards = self.view_stats_cards();

        // Main content area with side-by-side layout
        let main_content = row![
            // Left panel: Groups and controls
            container(self.view_control_panel())
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::FillPortion(1)),
            // Right panel: Results and monitoring
            container(self.view_results_panel())
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::FillPortion(2))
        ]
        .spacing(theme::layout::SPACING_MD);

        let content = column![header, stats_cards, main_content]
            .spacing(theme::layout::SPACING_MD)
            .padding(theme::layout::PADDING_MD);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_header(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();

        let title_row = row![
            column![
                theme::typography::title("⚡ BTC Farm Management"),
                theme::typography::small("Bitcoin ASIC Miner Control Center")
            ]
            .spacing(theme::layout::SPACING_XS),
            Space::new(Length::Fill, Length::Fixed(0.0)),
            // System status indicator
            container(
                row![
                    text("🟢").size(16),
                    theme::typography::body("System Online")
                ]
                .spacing(theme::layout::SPACING_XS)
                .align_y(iced::alignment::Vertical::Center)
            )
            .style(iced::widget::container::rounded_box)
            .padding([theme::layout::PADDING_XS, theme::layout::PADDING_SM])
        ]
        .align_y(iced::alignment::Vertical::Center);

        container(title_row)
            .style(theme::container_styles::header)
            .padding(theme::layout::PADDING_MD)
            .width(Length::Fill)
            .into()
    }

    fn view_stats_cards(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();
        let enabled_groups = self.app_config.get_enabled_groups();
        let all_results = self.app_config.get_all_scan_results();
        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();
        let total_ips: usize = enabled_groups
            .iter()
            .map(|group| self.estimate_ip_count(&group.network_range))
            .sum();

        let stats = row![
            // Total groups card
            container(
                column![
                    theme::typography::mono_large(self.app_config.scan_groups.len().to_string()),
                    theme::typography::small("Total Groups"),
                    theme::typography::tiny(format!("{} enabled", enabled_groups.len()))
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // IP ranges card
            container(
                column![
                    theme::typography::mono_large(format!("~{total_ips}")),
                    theme::typography::small("IP Addresses"),
                    theme::typography::tiny("to scan")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // Discovered miners card
            container(
                column![
                    theme::typography::mono_large(total_miners.to_string()),
                    theme::typography::small("Miners Found"),
                    theme::typography::tiny("last scan")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1)),
            // Scan status card
            container(
                column![
                    text(if self.scanning { "🔄" } else { "⏸️" }).size(24),
                    theme::typography::small(if self.scanning { "Scanning" } else { "Ready" }),
                    theme::typography::tiny(if self.scanning { "in progress" } else { "idle" })
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_XS)
            )
            .style(if self.scanning {
                theme::container_styles::status_warning
            } else {
                theme::container_styles::card
            })
            .padding(theme::layout::PADDING_MD)
            .width(Length::FillPortion(1))
        ]
        .spacing(theme::layout::SPACING_MD);

        stats.into()
    }

    fn view_control_panel(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();
        let enabled_groups = self.app_config.get_enabled_groups();

        let panel_header = row![
            theme::typography::heading("🔧 Control Panel"),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ];

        let controls = column![
            // Configuration button
            button(
                row![
                    text("⚙️").size(16),
                    theme::typography::body("Configure Groups")
                ]
                .spacing(theme::layout::SPACING_SM)
                .align_y(iced::alignment::Vertical::Center)
            )
            .style(iced::widget::button::secondary)
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill)
            .on_press(DashboardMessage::OpenNetworkConfig),
            // Scan button
            {
                if enabled_groups.is_empty() {
                    button(theme::typography::body("❌ No Groups Enabled"))
                        .style(iced::widget::button::secondary)
                        .padding(theme::layout::PADDING_SM)
                        .width(Length::Fill)
                } else if self.scanning {
                    button(theme::typography::body("⏹️ Stop Scan"))
                        .style(iced::widget::button::danger)
                        .padding(theme::layout::PADDING_SM)
                        .width(Length::Fill)
                        .on_press(DashboardMessage::StopScan)
                } else {
                    button(theme::typography::body("▶️ Start Scan"))
                        .style(iced::widget::button::primary)
                        .padding(theme::layout::PADDING_SM)
                        .width(Length::Fill)
                        .on_press(DashboardMessage::StartScan)
                }
            }
        ]
        .spacing(theme::layout::SPACING_MD);

        // Groups overview
        let groups_section = self.view_groups_overview();

        column![
            panel_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
            controls,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_LG)),
            groups_section
        ]
        .spacing(theme::layout::SPACING_SM)
        .into()
    }

    fn view_results_panel(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();

        let panel_header = row![
            theme::typography::heading("📊 Mining Operations"),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ];

        let results_content = self.view_scan_results();

        column![
            panel_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
            results_content
        ]
        .spacing(theme::layout::SPACING_SM)
        .into()
    }

    fn view_groups_overview(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();

        if self.app_config.scan_groups.is_empty() {
            return container(
                column![
                    text("📁").size(32),
                    theme::typography::body("No scan groups configured"),
                    theme::typography::small("Use 'Configure Groups' to add network ranges")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_SM),
            )
            .padding(theme::layout::PADDING_LG)
            .into();
        }

        let header = theme::typography::heading("📁 Scan Groups");

        let mut groups_list = column![].spacing(theme::layout::SPACING_SM);

        for group in &self.app_config.scan_groups {
            let status_icon = if group.enabled { "🟢" } else { "🔴" };
            let estimated_ips = self.estimate_ip_count(&group.network_range);

            let group_card = container(
                row![
                    column![
                        row![
                            text(status_icon).size(14),
                            theme::typography::body(&group.name)
                        ]
                        .spacing(theme::layout::SPACING_XS)
                        .align_y(iced::alignment::Vertical::Center),
                        theme::typography::mono(&group.network_range),
                        theme::typography::tiny(format!("~{estimated_ips} IPs"))
                    ]
                    .spacing(theme::layout::SPACING_XS)
                    .width(Length::Fill),
                    container(theme::typography::small(if group.enabled {
                        "ENABLED"
                    } else {
                        "DISABLED"
                    }))
                    .style(if group.enabled {
                        theme::container_styles::status_success
                    } else {
                        theme::container_styles::card
                    })
                    .padding([theme::layout::PADDING_XS, theme::layout::PADDING_SM])
                ]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(theme::layout::SPACING_SM),
            )
            .style(iced::widget::container::rounded_box)
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill);

            groups_list = groups_list.push(group_card);
        }

        column![
            header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),
            scrollable(groups_list).height(Length::Fixed(200.0))
        ]
        .spacing(theme::layout::SPACING_XS)
        .into()
    }

    fn view_scan_results(&self) -> Element<DashboardMessage> {
        let _theme = BtcTheme::default();
        let all_results = self.app_config.get_all_scan_results();

        if all_results.is_empty() {
            return container(
                column![
                    text("⛏️").size(32),
                    theme::typography::body("No miners discovered yet"),
                    theme::typography::small("Run a scan to find ASIC miners on your network")
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

        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();

        // Header with summary
        let summary_header = container(row![
            column![
                theme::typography::heading(format!("⛏️ {total_miners} Miners Online")),
                theme::typography::small(format!("Across {} groups", all_results.len()))
            ]
            .spacing(theme::layout::SPACING_XS),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ])
        .style(iced::widget::container::rounded_box)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        let mut results_content = column![].spacing(theme::layout::SPACING_MD);

        for (group_name, miners) in all_results.iter() {
            if miners.is_empty() {
                continue;
            }

            // Group section header
            let group_header = container(
                row![
                    theme::typography::heading(format!("📊 {group_name}")),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                    container(theme::typography::small(format!("{} miners", miners.len())))
                        .style(theme::container_styles::status_success)
                        .padding([theme::layout::PADDING_XS, theme::layout::PADDING_SM])
                ]
                .align_y(iced::alignment::Vertical::Center),
            )
            .padding(theme::layout::PADDING_SM)
            .width(Length::Fill);

            // Miners table header
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

            // Sort miners by IP address for consistent display
            let mut sorted_miners = miners.clone();
            sorted_miners.sort_by_key(|m| m.ip);

            for miner in sorted_miners {
                let miner_ip = miner.ip;

                let miner_row = container(
                    row![
                        button(theme::typography::mono(miner_ip.to_string()))
                            .style(iced::widget::button::text)
                            .padding(theme::layout::PADDING_XS)
                            .width(Length::FillPortion(3))
                            .on_press(DashboardMessage::OpenIpInBrowser(miner_ip)),
                        theme::typography::body(&miner.model).width(Length::FillPortion(3)),
                        theme::typography::body(miner.make.as_deref().unwrap_or("Unknown"))
                            .width(Length::FillPortion(2)),
                        theme::typography::body(miner.firmware.as_deref().unwrap_or("Unknown"))
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
                scrollable(miners_list).height(Length::Fixed(200.0))
            ]
            .spacing(theme::layout::SPACING_XS);

            results_content = results_content.push(group_section);
        }

        column![
            summary_header,
            scrollable(results_content).height(Length::Fill)
        ]
        .spacing(theme::layout::SPACING_MD)
        .into()
    }
}
