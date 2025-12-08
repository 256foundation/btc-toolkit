use crate::config::AppConfig;
use crate::network::estimate_ip_count;
use crate::sorting::{SortColumn, SortDirection, sort_miners_by_column};
use crate::theme;
use crate::ui_helpers::{
    calculate_progress, danger_button, format_duration, primary_button, secondary_button,
};
use asic_rs::data::miner::MinerData;
use iced::widget::{Space, button, column, container, progress_bar, row, scrollable};
use iced::{Element, Length, Task};
use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum MainViewMessage {
    OpenNetworkConfig,
    StartScan,
    StopScan,
    AddGroup,
    OpenIpInBrowser(Ipv4Addr),
    OpenDeviceDetail(Ipv4Addr),
    MinerFound {
        group_name: String,
        miner: MinerData,
    },
    IpScanned {
        group_name: String,
        total_ips: usize,
        scanned_count: usize,
    },
    GroupCompleted(String),
    GroupError {
        group_name: String,
        error: String,
    },
    AllScansCompleted,
    SortColumn(SortColumn),
    ToggleGroupCollapse(String),
}

#[derive(Debug, Clone)]
pub struct GroupScanStatus {
    pub completed: bool,
    pub error: Option<String>,
    pub miner_count: usize,
    pub total_ips: usize,
    pub scanned_ips: usize,
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
    collapsed_groups: HashSet<String>,
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
            collapsed_groups: HashSet::new(),
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
            MainViewMessage::AddGroup => Task::none(),
            MainViewMessage::OpenIpInBrowser(ip) => {
                let url = format!("http://{}", ip);
                if let Err(e) = opener::open(&url) {
                    eprintln!("Failed to open URL {}: {}", url, e);
                }
                Task::none()
            }
            MainViewMessage::OpenDeviceDetail(_ip) => {
                // This is handled at the BtcToolkit level, not here
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
                            total_ips: 0, // Will be set when first IpScanned message arrives
                            scanned_ips: 0,
                        },
                    );
                }
                Task::none()
            }
            MainViewMessage::IpScanned {
                group_name,
                total_ips,
                scanned_count,
            } => {
                if let Some(status) = self.group_status.get_mut(&group_name) {
                    status.total_ips = total_ips;
                    status.scanned_ips = scanned_count;
                } else {
                    self.group_status.insert(
                        group_name,
                        GroupScanStatus {
                            completed: false,
                            error: None,
                            miner_count: 0,
                            total_ips,
                            scanned_ips: scanned_count,
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

                let existing_status = self.group_status.get(&group_name);
                let (total_ips, scanned_ips) = existing_status
                    .map(|s| (s.total_ips, s.scanned_ips))
                    .unwrap_or((0, 0));

                self.group_status.insert(
                    group_name.clone(),
                    GroupScanStatus {
                        completed: true,
                        error: None,
                        miner_count,
                        total_ips,
                        scanned_ips,
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
                let existing_status = self.group_status.get(&group_name);
                let (total_ips, scanned_ips) = existing_status
                    .map(|s| (s.total_ips, s.scanned_ips))
                    .unwrap_or((0, 0));

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
                        total_ips,
                        scanned_ips,
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
                    self.sort_direction = self.sort_direction.toggle();
                } else {
                    self.sort_column = Some(column);
                    self.sort_direction = SortDirection::Ascending;
                }
                Task::none()
            }
            MainViewMessage::ToggleGroupCollapse(group_name) => {
                if self.collapsed_groups.contains(&group_name) {
                    self.collapsed_groups.remove(&group_name);
                } else {
                    self.collapsed_groups.insert(group_name);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, MainViewMessage> {
        let toolbar = self.view_toolbar();
        let stats = self.view_stats();
        let main_content = self.view_main_content();

        // Compact header: stats on left, controls on right
        let header = container(
            row![stats, Space::new().width(Length::Fill), toolbar]
                .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::SM)
        .width(Length::Fill);

        container(
            column![header, main_content]
                .spacing(theme::spacing::SM)
                .padding(theme::padding::SM),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn view_toolbar(&self) -> Element<'_, MainViewMessage> {
        let scan_button = if self.is_scanning {
            danger_button(
                "Stop",
                Some(theme::icons::stop().into()),
                Some(MainViewMessage::StopScan),
            )
        } else {
            let enabled_groups = self.app_config.get_enabled_groups();
            if enabled_groups.is_empty() {
                secondary_button("No Groups", None, None)
            } else {
                primary_button(
                    "Scan",
                    Some(theme::icons::play().into()),
                    Some(MainViewMessage::StartScan),
                )
            }
        };

        let config_button = secondary_button(
            "Config",
            Some(theme::icons::settings().into()),
            Some(MainViewMessage::OpenNetworkConfig),
        );

        row![scan_button, config_button]
            .spacing(theme::spacing::SM)
            .into()
    }

    fn view_stats(&self) -> Element<'_, MainViewMessage> {
        let enabled_groups = self.app_config.get_enabled_groups();
        let all_results = if self.is_scanning {
            &self.discovered_miners_by_group
        } else {
            self.app_config.get_all_scan_results()
        };

        let total_miners: usize = all_results.values().map(|miners| miners.len()).sum();
        let total_ips: usize = enabled_groups
            .iter()
            .map(|group| estimate_ip_count(&group.network_range))
            .sum();

        // Compact inline stats bar
        let stats_row = if self.is_scanning {
            let (total_ips_all_groups, scanned_ips_all_groups) =
                self.group_status
                    .values()
                    .fold((0, 0), |(total_acc, scanned_acc), status| {
                        (
                            total_acc + status.total_ips,
                            scanned_acc + status.scanned_ips,
                        )
                    });

            let progress_value = if total_ips_all_groups > 0 {
                calculate_progress(scanned_ips_all_groups, total_ips_all_groups)
            } else {
                calculate_progress(self.completed_groups, self.total_groups)
            };

            let elapsed =
                format_duration(self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0));

            row![
                theme::typography::small(format!("{} miners found", total_miners)),
                Space::new().width(theme::spacing::MD),
                theme::typography::small(format!(
                    "{}/{} IPs",
                    scanned_ips_all_groups, total_ips_all_groups
                )),
                Space::new().width(theme::spacing::SM),
                container(progress_bar(0.0..=1.0, progress_value)).width(Length::Fixed(120.0)),
                Space::new().width(theme::spacing::SM),
                theme::typography::tiny(elapsed),
            ]
            .align_y(iced::alignment::Vertical::Center)
        } else {
            row![
                theme::typography::small(format!(
                    "{} groups ({} enabled)",
                    self.app_config.scan_groups.len(),
                    enabled_groups.len()
                )),
                Space::new().width(theme::spacing::MD),
                theme::typography::small(format!("~{} IPs", total_ips)),
                Space::new().width(theme::spacing::MD),
                theme::typography::small(format!("{} miners", total_miners)),
            ]
            .align_y(iced::alignment::Vertical::Center)
        };

        stats_row.into()
    }

    fn view_main_content(&self) -> Element<'_, MainViewMessage> {
        // Get results from current scan or last scan
        let results = if self.is_scanning {
            &self.discovered_miners_by_group
        } else {
            self.app_config.get_all_scan_results()
        };

        if self.app_config.scan_groups.is_empty() {
            return container(
                column![
                    theme::typography::small("No groups configured"),
                    theme::typography::tiny("Use Config to add network ranges")
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::spacing::XS),
            )
            .padding(theme::padding::MD)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        let mut content = column![].spacing(theme::spacing::SM);

        for group in &self.app_config.scan_groups {
            let estimated_ips = estimate_ip_count(&group.network_range);
            let status = self.group_status.get(&group.name);
            let miners = results.get(&group.name);
            let miner_count = miners.map(|m| m.len()).unwrap_or(0);
            let is_collapsed = self.collapsed_groups.contains(&group.name);

            // Group status text
            let status_text = if let Some(status) = status {
                if status.completed {
                    if status.error.is_some() {
                        "error".to_string()
                    } else {
                        format!("{} miners", status.miner_count)
                    }
                } else if status.total_ips > 0 {
                    format!("scanning {}/{}", status.scanned_ips, status.total_ips)
                } else {
                    "scanning...".to_string()
                }
            } else if miner_count > 0 {
                format!("{} miners", miner_count)
            } else if group.enabled {
                "ready".to_string()
            } else {
                "disabled".to_string()
            };

            // Collapse indicator
            let collapse_icon = if is_collapsed { "▶" } else { "▼" };

            // Group header (clickable)
            let group_header = button(
                container(
                    row![
                        theme::typography::body(collapse_icon),
                        Space::new().width(theme::spacing::SM),
                        theme::typography::body(&group.name),
                        Space::new().width(theme::spacing::MD),
                        theme::typography::small(&group.network_range),
                        theme::typography::small(format!(" (~{})", estimated_ips)),
                        Space::new().width(Length::Fill),
                        theme::typography::body(status_text)
                    ]
                    .align_y(iced::alignment::Vertical::Center),
                )
                .style(theme::containers::header)
                .padding([theme::padding::SM, theme::padding::MD])
                .width(Length::Fill),
            )
            .style(button::text)
            .padding(0)
            .on_press(MainViewMessage::ToggleGroupCollapse(group.name.clone()))
            .width(Length::Fill);

            // Miners list for this group (only if not collapsed)
            let group_section = if is_collapsed {
                column![group_header]
            } else {
                let miners_content: Element<'_, MainViewMessage> = if let Some(miners) = miners {
                    if miners.is_empty() {
                        container(theme::typography::tiny("No miners found"))
                            .padding([theme::padding::XS, theme::padding::MD])
                            .into()
                    } else {
                        let mut sorted_miners = miners.clone();
                        self.sort_miners(&mut sorted_miners);

                        // Table header with sortable columns
                        let sort_arrow = |col: SortColumn| -> String {
                            if self.sort_column == Some(col) {
                                match self.sort_direction {
                                    SortDirection::Ascending => " ▲".to_string(),
                                    SortDirection::Descending => " ▼".to_string(),
                                }
                            } else {
                                String::new()
                            }
                        };

                        let table_header = container(
                            row![
                                container(
                                    button(theme::typography::small(format!(
                                        "IP{}",
                                        sort_arrow(SortColumn::IpAddress)
                                    )))
                                    .style(button::text)
                                    .padding(0)
                                    .on_press(MainViewMessage::SortColumn(SortColumn::IpAddress))
                                )
                                .width(Length::FillPortion(2)),
                                container(
                                    button(theme::typography::small(format!(
                                        "Model{}",
                                        sort_arrow(SortColumn::Model)
                                    )))
                                    .style(button::text)
                                    .padding(0)
                                    .on_press(MainViewMessage::SortColumn(SortColumn::Model))
                                )
                                .width(Length::FillPortion(2)),
                                container(
                                    button(theme::typography::small(format!(
                                        "Make{}",
                                        sort_arrow(SortColumn::Make)
                                    )))
                                    .style(button::text)
                                    .padding(0)
                                    .on_press(MainViewMessage::SortColumn(SortColumn::Make))
                                )
                                .width(Length::FillPortion(1)),
                                container(
                                    button(theme::typography::small(format!(
                                        "Firmware{}",
                                        sort_arrow(SortColumn::Firmware)
                                    )))
                                    .style(button::text)
                                    .padding(0)
                                    .on_press(MainViewMessage::SortColumn(SortColumn::Firmware))
                                )
                                .width(Length::FillPortion(1)),
                                container(
                                    button(theme::typography::small(format!(
                                        "Version{}",
                                        sort_arrow(SortColumn::FirmwareVersion)
                                    )))
                                    .style(button::text)
                                    .padding(0)
                                    .on_press(
                                        MainViewMessage::SortColumn(SortColumn::FirmwareVersion)
                                    )
                                )
                                .width(Length::FillPortion(1)),
                            ]
                            .spacing(theme::spacing::XS),
                        )
                        .padding(theme::padding::XS);

                        let mut miners_list = column![].spacing(2.0);

                        for miner in sorted_miners {
                            let miner_ip = match miner.ip {
                                std::net::IpAddr::V4(ipv4) => ipv4,
                                std::net::IpAddr::V6(_) => continue,
                            };

                            let miner_row = button(
                                row![
                                    container(theme::typography::mono(miner_ip.to_string()))
                                        .width(Length::FillPortion(2)),
                                    container(theme::typography::mono(
                                        format!("{}", miner.device_info.model).replace("Plus", "+")
                                    ))
                                    .width(Length::FillPortion(2)),
                                    container(theme::typography::mono(format!(
                                        "{}",
                                        miner.device_info.make
                                    )))
                                    .width(Length::FillPortion(1)),
                                    container(theme::typography::mono(format!(
                                        "{}",
                                        miner.device_info.firmware
                                    )))
                                    .width(Length::FillPortion(1)),
                                    container(theme::typography::mono(
                                        miner.firmware_version.as_deref().unwrap_or("-")
                                    ))
                                    .width(Length::FillPortion(1)),
                                ]
                                .spacing(theme::spacing::XS)
                                .align_y(iced::alignment::Vertical::Center),
                            )
                            .style(theme::buttons::table_row)
                            .padding(theme::padding::XS)
                            .on_press(MainViewMessage::OpenDeviceDetail(miner_ip))
                            .width(Length::Fill);

                            miners_list = miners_list.push(miner_row);
                        }

                        container(column![table_header, miners_list].spacing(theme::spacing::XS))
                            .padding([0.0, theme::padding::MD])
                            .into()
                    }
                } else {
                    container(theme::typography::tiny("Not scanned"))
                        .padding([theme::padding::XS, theme::padding::MD])
                        .into()
                };

                column![group_header, miners_content].spacing(theme::spacing::XS)
            };

            content = content.push(group_section);
        }

        container(scrollable(content))
            .style(theme::containers::card)
            .padding(theme::padding::SM)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn sort_miners(&self, miners: &mut [MinerData]) {
        if let Some(column) = self.sort_column {
            sort_miners_by_column(miners, column, self.sort_direction);
        }
    }
}
