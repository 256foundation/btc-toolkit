use crate::config::AppConfig;
use crate::network::estimate_ip_count;
use crate::theme;
use asic_rs::data::miner::MinerData;
use iced::widget::{Space, button, column, container, progress_bar, row, scrollable, text};
use iced::{Element, Length, Task};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum MainViewMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
    AddGroup,
    OpenIpInBrowser(Ipv4Addr),
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
pub struct GroupScanStatus {
    pub completed: bool,
    pub error: Option<String>,
    pub miner_count: usize,
}

pub struct MainView {
    app_config: AppConfig,
    is_scanning: bool,
    discovered_miners_by_group: HashMap<String, Vec<MinerData>>,
    group_status: HashMap<String, GroupScanStatus>,
    total_groups: usize,
    completed_groups: usize,
    start_time: Option<Instant>,
    total_ips_to_scan: usize,
    error_messages: Vec<String>,
    sort_column: Option<SortColumn>,
    sort_direction: SortDirection,
}

impl MainView {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        Self {
            app_config,
            is_scanning: false,
            discovered_miners_by_group: HashMap::new(),
            group_status: HashMap::new(),
            total_groups: 0,
            completed_groups: 0,
            start_time: None,
            total_ips_to_scan: 0,
            error_messages: Vec::new(),
            sort_column: Some(SortColumn::IpAddress),
            sort_direction: SortDirection::Ascending,
        }
    }

    pub fn set_app_config(&mut self, config: AppConfig) {
        self.app_config = config;
    }

    pub fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub fn start_scanning(&mut self, groups: Vec<String>) {
        self.is_scanning = true;
        self.start_time = Some(Instant::now());
        self.total_groups = groups.len();
        self.completed_groups = 0;
        self.discovered_miners_by_group.clear();
        self.group_status.clear();
        self.error_messages.clear();
        // Clear previous scan results from config
        self.app_config.clear_scan_results();

        let enabled_groups = self.app_config.get_enabled_groups();
        self.total_ips_to_scan = enabled_groups
            .iter()
            .map(|group| estimate_ip_count(&group.network_range))
            .sum();
    }

    pub fn update(&mut self, message: MainViewMessage) -> Task<MainViewMessage> {
        match message {
            MainViewMessage::OpenNetworkConfig => Task::none(),
            MainViewMessage::StartScan => {
                if !self.is_scanning {
                    let enabled_groups = self.app_config.get_enabled_groups();
                    let group_names: Vec<String> =
                        enabled_groups.iter().map(|g| g.name.clone()).collect();
                    self.start_scanning(group_names);
                }
                Task::none()
            }
            MainViewMessage::StopScan => {
                self.is_scanning = false;
                Task::none()
            }
            MainViewMessage::AddGroup => {
                // This will navigate to the network config page
                Task::none()
            }
            MainViewMessage::OpenIpInBrowser(ip) => {
                let url = format!("http://{}", ip);
                if let Err(e) = opener::open(&url) {
                    eprintln!("Failed to open URL {}: {}", url, e);
                }
                Task::none()
            }
            MainViewMessage::MinerFound { group_name, miner } => {
                self.discovered_miners_by_group
                    .entry(group_name.clone())
                    .or_default()
                    .push(miner);

                if let Some(status) = self.group_status.get_mut(&group_name) {
                    status.miner_count += 1;
                } else {
                    self.group_status.insert(
                        group_name,
                        GroupScanStatus {
                            completed: false,
                            error: None,
                            miner_count: 1,
                        },
                    );
                }
                Task::none()
            }
            MainViewMessage::GroupCompleted(group_name) => {
                let miner_count = self
                    .discovered_miners_by_group
                    .get(&group_name)
                    .map(|miners| miners.len())
                    .unwrap_or(0);

                self.group_status.insert(
                    group_name.clone(),
                    GroupScanStatus {
                        completed: true,
                        error: None,
                        miner_count,
                    },
                );
                self.completed_groups += 1;

                self.app_config.store_scan_results(
                    &group_name,
                    self.discovered_miners_by_group
                        .get(&group_name)
                        .cloned()
                        .unwrap_or_default(),
                );

                if let Err(e) = self.app_config.save() {
                    eprintln!("Failed to save config: {}", e);
                }

                Task::none()
            }
            MainViewMessage::GroupError { group_name, error } => {
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
                self.error_messages
                    .push(format!("{}: {}", group_name, error));
                self.completed_groups += 1;
                Task::none()
            }
            MainViewMessage::AllScansCompleted => {
                self.is_scanning = false;
                Task::none()
            }
            MainViewMessage::SortColumn(column) => {
                if Some(column) == self.sort_column {
                    self.sort_direction = match self.sort_direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };
                } else {
                    self.sort_column = Some(column);
                    self.sort_direction = SortDirection::Ascending;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainViewMessage> {
        let toolbar = self.view_toolbar();
        let stats = self.view_stats();
        let main_content = self.view_main_content();

        container(
            column![toolbar, stats, main_content]
                .spacing(theme::spacing::MD)
                .padding(theme::padding::MD),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_toolbar(&self) -> Element<'_, MainViewMessage> {
        let title = theme::typography::title("BTC Farm Management");
        let subtitle = theme::typography::small("Bitcoin ASIC Miner Control Center");

        let scan_button = if self.is_scanning {
            button(
                row![text("⬛"), theme::typography::body("Stop Scan")]
                    .spacing(theme::spacing::XS)
                    .align_y(iced::alignment::Vertical::Center),
            )
            .style(button::danger)
            .padding(theme::padding::SM)
            .on_press(MainViewMessage::StopScan)
        } else {
            let enabled_groups = self.app_config.get_enabled_groups();
            if enabled_groups.is_empty() {
                button(theme::typography::body("No Groups Enabled"))
                    .style(button::secondary)
                    .padding(theme::padding::SM)
            } else {
                button(
                    row![text("▶"), theme::typography::body("Start Scan")]
                        .spacing(theme::spacing::XS)
                        .align_y(iced::alignment::Vertical::Center),
                )
                .style(button::primary)
                .padding(theme::padding::SM)
                .on_press(MainViewMessage::StartScan)
            }
        };

        let add_group_button = button(
            row![text("➕"), theme::typography::body("Add Group")]
                .spacing(theme::spacing::XS)
                .align_y(iced::alignment::Vertical::Center),
        )
        .style(button::secondary)
        .padding(theme::padding::SM)
        .on_press(MainViewMessage::AddGroup);

        let config_button = button(
            row![text("⚙"), theme::typography::body("Configure")]
                .spacing(theme::spacing::XS)
                .align_y(iced::alignment::Vertical::Center),
        )
        .style(button::secondary)
        .padding(theme::padding::SM)
        .on_press(MainViewMessage::OpenNetworkConfig);

        container(
            row![
                column![title, subtitle].spacing(theme::spacing::XS),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                row![scan_button, add_group_button, config_button].spacing(theme::spacing::SM)
            ]
            .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::MD)
        .width(Length::Fill)
        .into()
    }

    fn view_stats(&self) -> Element<'_, MainViewMessage> {
        let enabled_groups = self.app_config.get_enabled_groups();
        let all_results = if self.is_scanning {
            &self.discovered_miners_by_group
        } else {
            &self.app_config.get_all_scan_results()
        };

        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();
        let total_ips: usize = enabled_groups
            .iter()
            .map(|group| estimate_ip_count(&group.network_range))
            .sum();

        let progress = if self.is_scanning && self.total_groups > 0 {
            let progress_value = self.completed_groups as f32 / self.total_groups as f32;
            container(
                column![
                    row![
                        theme::typography::body(format!("Scanning {} groups", self.total_groups)),
                        Space::new(Length::Fill, Length::Fixed(0.0)),
                        theme::typography::body(format!(
                            "{}/{} completed",
                            self.completed_groups, self.total_groups
                        ))
                    ],
                    progress_bar(0.0..=1.0, progress_value)
                ]
                .spacing(theme::spacing::XS),
            )
            .style(theme::containers::warning)
            .padding(theme::padding::MD)
            .width(Length::Fill)
        } else {
            container(Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
        };

        column![
            row![
                container(
                    column![
                        theme::typography::mono_large(
                            self.app_config.scan_groups.len().to_string()
                        ),
                        theme::typography::small("Total Groups"),
                        theme::typography::tiny(format!("{} enabled", enabled_groups.len()))
                    ]
                    .align_x(iced::alignment::Horizontal::Center)
                    .spacing(theme::spacing::XS)
                )
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1)),
                container(
                    column![
                        theme::typography::mono_large(format!("~{}", total_ips)),
                        theme::typography::small("IP Addresses"),
                        theme::typography::tiny("to scan")
                    ]
                    .align_x(iced::alignment::Horizontal::Center)
                    .spacing(theme::spacing::XS)
                )
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1)),
                container(
                    column![
                        theme::typography::mono_large(total_miners.to_string()),
                        theme::typography::small("Miners Found"),
                        theme::typography::tiny(if self.is_scanning {
                            "current scan"
                        } else {
                            "last scan"
                        })
                    ]
                    .align_x(iced::alignment::Horizontal::Center)
                    .spacing(theme::spacing::XS)
                )
                .style(theme::containers::card)
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1)),
                container(
                    column![
                        theme::typography::mono_large(if self.is_scanning {
                            "Scanning"
                        } else {
                            "Ready"
                        }),
                        theme::typography::small("Status"),
                        theme::typography::tiny(if self.is_scanning {
                            format!(
                                "{}s",
                                self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0)
                            )
                        } else {
                            "idle".to_string()
                        })
                    ]
                    .align_x(iced::alignment::Horizontal::Center)
                    .spacing(theme::spacing::XS)
                )
                .style(if self.is_scanning {
                    theme::containers::warning
                } else {
                    theme::containers::card
                })
                .padding(theme::padding::MD)
                .width(Length::FillPortion(1))
            ]
            .spacing(theme::spacing::MD),
            progress
        ]
        .spacing(theme::spacing::MD)
        .into()
    }

    fn view_main_content(&self) -> Element<'_, MainViewMessage> {
        let left_panel = container(self.view_groups_panel())
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let right_panel = container(self.view_results_panel())
            .style(theme::containers::card)
            .padding(theme::padding::MD)
            .width(Length::FillPortion(2))
            .height(Length::Fill);

        row![left_panel, right_panel]
            .spacing(theme::spacing::MD)
            .height(Length::Fill)
            .into()
    }

    fn view_groups_panel(&self) -> Element<'_, MainViewMessage> {
        let header = theme::typography::heading("Scan Groups");

        if self.app_config.scan_groups.is_empty() {
            return column![
                header,
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::MD)),
                container(
                    column![
                        theme::typography::body("No scan groups configured"),
                        theme::typography::small(
                            "Use 'Add Group' or 'Configure' to add network ranges"
                        )
                    ]
                    .align_x(iced::alignment::Horizontal::Center)
                    .spacing(theme::spacing::SM)
                )
                .padding(theme::padding::LG)
            ]
            .into();
        }

        let mut groups_list = column![].spacing(theme::spacing::SM);

        for group in &self.app_config.scan_groups {
            let estimated_ips = estimate_ip_count(&group.network_range);
            let status = self.group_status.get(&group.name);

            let status_badge = if let Some(status) = status {
                if status.completed {
                    if status.error.is_some() {
                        container(theme::typography::small("ERROR"))
                            .style(theme::containers::card)
                            .padding([theme::padding::XS, theme::padding::SM])
                    } else {
                        container(theme::typography::small(format!(
                            "{} miners",
                            status.miner_count
                        )))
                        .style(theme::containers::success)
                        .padding([theme::padding::XS, theme::padding::SM])
                    }
                } else {
                    container(theme::typography::small("SCANNING"))
                        .style(theme::containers::warning)
                        .padding([theme::padding::XS, theme::padding::SM])
                }
            } else if group.enabled {
                container(theme::typography::small("ENABLED"))
                    .style(theme::containers::success)
                    .padding([theme::padding::XS, theme::padding::SM])
            } else {
                container(theme::typography::small("DISABLED"))
                    .style(theme::containers::card)
                    .padding([theme::padding::XS, theme::padding::SM])
            };

            let group_card = container(
                column![
                    row![
                        theme::typography::body(&group.name),
                        Space::new(Length::Fill, Length::Fixed(0.0)),
                        status_badge
                    ]
                    .align_y(iced::alignment::Vertical::Center),
                    theme::typography::mono(&group.network_range),
                    theme::typography::tiny(format!("~{} IPs", estimated_ips))
                ]
                .spacing(theme::spacing::XS),
            )
            .style(theme::containers::card)
            .padding(theme::padding::SM)
            .width(Length::Fill);

            groups_list = groups_list.push(group_card);
        }

        column![
            header,
            Space::new(Length::Fixed(0.0), Length::Fixed(theme::spacing::SM)),
            scrollable(groups_list)
        ]
        .spacing(theme::spacing::XS)
        .into()
    }

    fn view_results_panel(&self) -> Element<'_, MainViewMessage> {
        let all_results = if self.is_scanning {
            &self.discovered_miners_by_group
        } else {
            &self.app_config.get_all_scan_results()
        };

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

        let header = container(row![
            column![
                theme::typography::heading(format!("{} Miners", total_miners)),
                theme::typography::small(format!("Across {} groups", all_results.len()))
            ]
            .spacing(theme::spacing::XS),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ])
        .style(theme::containers::card)
        .padding(theme::padding::MD)
        .width(Length::Fill);

        let table_header = container(
            row![
                container(
                    button(theme::typography::small("IP Address"))
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::SortColumn(SortColumn::IpAddress))
                )
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(3))
                .padding(theme::padding::XS),
                container(
                    button(theme::typography::small("Model"))
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::SortColumn(SortColumn::Model))
                )
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(3))
                .padding(theme::padding::XS),
                container(
                    button(theme::typography::small("Make"))
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::SortColumn(SortColumn::Make))
                )
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(2))
                .padding(theme::padding::XS),
                container(
                    button(theme::typography::small("Firmware"))
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::SortColumn(SortColumn::Firmware))
                )
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(2))
                .padding(theme::padding::XS),
                container(
                    button(theme::typography::small("Firmware Version"))
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::SortColumn(SortColumn::FirmwareVersion))
                )
                .align_x(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(2))
                .padding(theme::padding::XS),
            ]
            .spacing(theme::spacing::SM),
        )
        .style(theme::containers::header)
        .padding(theme::padding::SM)
        .width(Length::Fill);

        let mut all_miners = Vec::new();
        for miners in all_results.values() {
            all_miners.extend(miners.clone());
        }

        self.sort_miners(&mut all_miners);

        let mut miners_list = column![]
            .spacing(theme::spacing::XS)
            .padding(theme::padding::SCROLLABLE);

        for miner in all_miners {
            let miner_ip = match miner.ip {
                std::net::IpAddr::V4(ipv4) => ipv4,
                std::net::IpAddr::V6(_) => continue,
            };

            let miner_row = container(
                row![
                    container(
                        button(
                            theme::typography::mono(miner_ip.to_string())
                        )
                        .style(button::text)
                        .padding(0)
                        .on_press(MainViewMessage::OpenIpInBrowser(miner_ip))
                    )
                    .align_x(iced::alignment::Horizontal::Left)
                    .width(Length::FillPortion(3))
                    .padding(theme::padding::XS),
                    container(
                        theme::typography::body(
                            format!("{}", miner.device_info.model).replace("Plus", "+")
                        )
                    )
                    .align_x(iced::alignment::Horizontal::Left)
                    .width(Length::FillPortion(3))
                    .padding(theme::padding::XS),
                    container(
                        theme::typography::body(format!("{}", miner.device_info.make))
                    )
                    .align_x(iced::alignment::Horizontal::Left)
                    .width(Length::FillPortion(2))
                    .padding(theme::padding::XS),
                    container(
                        theme::typography::body(format!("{}", miner.device_info.firmware))
                    )
                    .align_x(iced::alignment::Horizontal::Left)
                    .width(Length::FillPortion(2))
                    .padding(theme::padding::XS),
                    container(
                        theme::typography::body(format!(
                            "{}",
                            miner.firmware_version.as_ref().unwrap_or(&"-".to_string())
                        ))
                    )
                    .align_x(iced::alignment::Horizontal::Left)
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

        column![header, table_header, scrollable(miners_list)]
            .spacing(theme::spacing::XS)
            .into()
    }

    fn sort_miners(&self, miners: &mut Vec<MinerData>) {
        match self.sort_column {
            Some(SortColumn::IpAddress) => {
                miners.sort_by_key(|m| m.ip);
                if self.sort_direction == SortDirection::Descending {
                    miners.reverse();
                }
            }
            Some(SortColumn::Model) => {
                miners.sort_by(|a, b| {
                    format!("{}", a.device_info.model).cmp(&format!("{}", b.device_info.model))
                });
                if self.sort_direction == SortDirection::Descending {
                    miners.reverse();
                }
            }
            Some(SortColumn::Make) => {
                miners.sort_by(|a, b| {
                    format!("{}", a.device_info.make).cmp(&format!("{}", b.device_info.make))
                });
                if self.sort_direction == SortDirection::Descending {
                    miners.reverse();
                }
            }
            Some(SortColumn::Firmware) => {
                miners.sort_by(|a, b| {
                    format!("{}", a.device_info.firmware)
                        .cmp(&format!("{}", b.device_info.firmware))
                });
                if self.sort_direction == SortDirection::Descending {
                    miners.reverse();
                }
            }
            Some(SortColumn::FirmwareVersion) => {
                miners.sort_by(|a, b| {
                    let a_version = a.firmware_version.as_deref().unwrap_or("");
                    let b_version = b.firmware_version.as_deref().unwrap_or("");
                    a_version.cmp(b_version)
                });
                if self.sort_direction == SortDirection::Descending {
                    miners.reverse();
                }
            }
            None => {}
        }
    }
}
