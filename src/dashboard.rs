use crate::config::AppConfig;
use crate::network::estimate_ip_count;
use crate::theme;
use iced::widget::{Space, button, column, container, row, scrollable};
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

    //noinspection HttpUrlsUsage
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

    pub fn view(&self) -> Element<'_, DashboardMessage> {
        // Header with title and system status
        let header = self.view_header();

        // Quick stats cards
        let stats_cards = self.view_stats_cards();

        // Main content area with side-by-side layout
        let main_content = row![
            // Left panel: Groups and controls
            container(self.view_control_panel())
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1)),
            // Right panel: Results and monitoring
            container(self.view_results_panel())
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(2))
        ]
        .spacing(theme::spacing::MD)
        .height(Length::Fill);

        let content = column![
            row![column![header]],
            row![
                column![stats_cards, main_content]
                    .spacing(theme::spacing::MD)
                    .padding(theme::padding::MD)
            ]
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_header(&self) -> Element<'_, DashboardMessage> {
        container(
            row![
                column![
                    theme::typography::title("BTC Farm Management"),
                    theme::typography::small("Bitcoin ASIC Miner Control Center")
                ]
                .spacing(theme::spacing::XS),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                // System status indicator
                container(
                    row![theme::typography::body("System Online")]
                        .spacing(theme::spacing::XS)
                        .align_y(iced::alignment::Vertical::Center)
                )
                .style(theme::containers::card)
                .padding(theme::padding::SM)
            ]
            .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::MD)
        .width(Length::Fill)
        .into()
    }

    fn view_stats_cards(&self) -> Element<'_, DashboardMessage> {
        let enabled_groups = self.app_config.get_enabled_groups();
        let all_results = self.app_config.get_all_scan_results();
        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();
        let total_ips: usize = enabled_groups
            .iter()
            .map(|group| estimate_ip_count(&group.network_range))
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
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::FillPortion(1)),
            // IP ranges card
            container(
                column![
                    theme::typography::mono_large(format!("~{total_ips}")),
                    theme::typography::small("IP Addresses"),
                    theme::typography::tiny("to scan")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::FillPortion(1)),
            // Discovered miners card
            container(
                column![
                    theme::typography::mono_large(total_miners.to_string()),
                    theme::typography::small("Miners Found"),
                    theme::typography::tiny("last scan")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS)
            )
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .align_x(iced::alignment::Horizontal::Center)
            .width(Length::FillPortion(1)),
            // Scan status card
            container(
                column![
                    theme::typography::mono_large(if self.scanning { "Scanning" } else { "Ready" }),
                    theme::typography::small("Status"),
                    theme::typography::tiny(if self.scanning { "in progress" } else { "idle" })
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS)
            )
            .style(if self.scanning {
                theme::containers::warning
            } else {
                theme::containers::card
            })
            .align_x(iced::alignment::Horizontal::Center)
            .padding(theme::padding::MD)
            .width(Length::FillPortion(1))
        ]
        .spacing(theme::spacing::MD);
        stats.into()
    }

    fn view_control_panel(&self) -> Element<'_, DashboardMessage> {
        let enabled_groups = self.app_config.get_enabled_groups();

        let panel_header = row![
            theme::typography::heading("Control Panel"),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ];

        let controls = column![
            // Configuration button
            button(
                row![
                    theme::typography::body("Configure Groups")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::Fill)
                ]
                .spacing(theme::spacing::SM)
                .align_y(iced::alignment::Vertical::Center)
            )
            .style(button::secondary)
            .padding(theme::padding::SM)
            .width(Length::Fill)
            .on_press(DashboardMessage::OpenNetworkConfig),
            // Scan button
            {
                if enabled_groups.is_empty() {
                    button(theme::typography::body("No Groups Enabled"))
                        .style(button::secondary)
                        .padding(theme::padding::SM)
                        .width(Length::Fill)
                } else if self.scanning {
                    button(theme::typography::body("Stop Scan"))
                        .style(button::danger)
                        .padding(theme::padding::SM)
                        .width(Length::Fill)
                        .on_press(DashboardMessage::StopScan)
                } else {
                    button(
                        theme::typography::body("Start Scan")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill),
                    )
                    .style(button::primary)
                    .padding(theme::padding::SM)
                    .width(Length::Fill)
                    .on_press(DashboardMessage::StartScan)
                }
            }
        ]
        .spacing(theme::spacing::MD);

        // Groups overview
        let groups_section = self.view_groups_overview();

        column![
            panel_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::MD)),
            controls,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::LG)),
            groups_section
        ]
        .spacing(theme::spacing::SM)
        .height(Length::Fill)
        .into()
    }

    fn view_results_panel(&self) -> Element<'_, DashboardMessage> {
        let panel_header = row![
            theme::typography::heading("Mining Operations"),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ];

        let results_content = self.view_scan_results();

        column![
            panel_header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::MD)),
            results_content
        ]
        .spacing(theme::spacing::SM)
        .height(Length::Fill)
        .into()
    }

    fn view_groups_overview(&self) -> Element<'_, DashboardMessage> {
        if self.app_config.scan_groups.is_empty() {
            return container(
                column![
                    theme::typography::body("No scan groups configured"),
                    theme::typography::small("Use 'Configure Groups' to add network ranges")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::SM),
            )
            .padding(theme::padding::LG)
            .into();
        }

        let header = theme::typography::heading("Scan Groups");

        let mut groups_list = column![].spacing(theme::spacing::SM);

        for group in &self.app_config.scan_groups {
            let estimated_ips = estimate_ip_count(&group.network_range);

            let group_card = container(
                row![
                    column![
                        row![theme::typography::body(&group.name)]
                            .spacing(theme::spacing::XS)
                            .align_y(iced::alignment::Vertical::Center),
                        theme::typography::mono(&group.network_range),
                        theme::typography::tiny(format!("~{estimated_ips} IPs"))
                    ]
                    .spacing(theme::spacing::XS)
                    .width(Length::Fill),
                    container(theme::typography::small(if group.enabled {
                        "ENABLED"
                    } else {
                        "DISABLED"
                    }))
                    .style(if group.enabled {
                        theme::containers::success
                    } else {
                        theme::containers::card
                    })
                    .padding([theme::padding::XS, theme::padding::SM])
                ]
                .align_y(iced::alignment::Vertical::Center)
                .spacing(theme::spacing::SM),
            )
            .style(theme::containers::card)
            .padding(theme::padding::SM)
            .width(Length::Fill);

            groups_list = groups_list.push(group_card);
        }

        column![
            header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::SM)),
            scrollable(groups_list).height(Length::Fixed(200.0))
        ]
        .spacing(theme::spacing::XS)
        .into()
    }

    fn view_scan_results(&self) -> Element<'_, DashboardMessage> {
        let all_results = self.app_config.get_all_scan_results();

        if all_results.is_empty() {
            return container(
                column![
                    theme::typography::body("No miners discovered yet"),
                    theme::typography::small("Run a scan to find ASIC miners on your network")
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

        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();

        // Header with summary
        let summary_header = container(row![
            column![
                theme::typography::heading(format!("{total_miners} Miners Online")),
                theme::typography::small(format!("Across {} groups", all_results.len()))
            ]
            .spacing(theme::spacing::XS),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ])
        .style(theme::containers::card)
        .padding(theme::padding::MD)
        .width(Length::Fill);

        let mut results_content = column![].spacing(theme::spacing::MD);

        for (group_name, miners) in all_results.iter() {
            if miners.is_empty() {
                continue;
            }

            // Group section header
            let group_header = container(
                row![
                    theme::typography::heading(format!("{group_name}")),
                    Space::new(Length::Fill, Length::Fixed(0.0)),
                    container(theme::typography::small(format!("{} miners", miners.len())))
                        .style(theme::containers::success)
                        .padding([theme::padding::XS, theme::padding::SM])
                ]
                .align_y(iced::alignment::Vertical::Center),
            )
            .padding(theme::padding::SM)
            .width(Length::Fill);

            // Miners table header
            let table_header = container(
                row![
                    theme::typography::small("IP Address")
                        .width(Length::FillPortion(3))
                        .align_x(iced::alignment::Horizontal::Center),
                    theme::typography::small("Model")
                        .width(Length::FillPortion(3))
                        .align_x(iced::alignment::Horizontal::Center),
                    theme::typography::small("Make")
                        .width(Length::FillPortion(2))
                        .align_x(iced::alignment::Horizontal::Center),
                    theme::typography::small("Firmware")
                        .width(Length::FillPortion(2))
                        .align_x(iced::alignment::Horizontal::Center),
                ]
                .spacing(theme::spacing::SM),
            )
            .style(theme::containers::header)
            .padding(theme::padding::SM)
            .width(Length::Fill);

            let mut miners_list = column![]
                .spacing(theme::spacing::XS)
                .padding(theme::padding::SCROLLABLE);

            // Sort miners by IP address for consistent display
            let mut sorted_miners = miners.clone();
            sorted_miners.sort_by_key(|m| m.ip);

            for miner in sorted_miners {
                let miner_ip = match miner.ip {
                    std::net::IpAddr::V4(ipv4) => ipv4,
                    std::net::IpAddr::V6(_) => continue, // Skip IPv6 addresses for now
                };

                let miner_row = container(
                    row![
                        button(
                            theme::typography::mono(miner_ip.to_string())
                                .align_x(iced::alignment::Horizontal::Center)
                        )
                        .style(button::text)
                        .padding(theme::padding::XS)
                        .width(Length::FillPortion(3))
                        .on_press(DashboardMessage::OpenIpInBrowser(miner_ip)),
                        theme::typography::body(
                            format!("{}", miner.device_info.model).replace("Plus", "+")
                        )
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(3)),
                        theme::typography::body(format!("{}", miner.device_info.make))
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::FillPortion(2)),
                        theme::typography::body(format!("{}", miner.device_info.firmware))
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::FillPortion(2)),
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

        column![summary_header, results_content]
            .spacing(theme::spacing::MD)
            .into()
    }
}
