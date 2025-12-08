use crate::errors::FetchError;
use crate::theme;
use crate::ui_helpers::{danger_button, format_duration, secondary_button};
use asic_rs::data::miner::MinerData;
use iced::Element;
use iced::Length;
use iced::widget::{Space, column, container, row, scrollable, text};
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub enum DeviceDetailMessage {
    Back,
    OpenInBrowser,
    Restart,
    SetPowerLimit,
    ToggleFaultLight,
    DataFetched(Result<MinerData, FetchError>),
}

pub enum DeviceDetailState {
    Loading(IpAddr),
    Loaded { miner: MinerData },
    Error(String),
}

pub struct DeviceDetailView {
    state: DeviceDetailState,
}

impl DeviceDetailView {
    pub fn new_loading(ip: IpAddr) -> Self {
        Self {
            state: DeviceDetailState::Loading(ip),
        }
    }

    pub fn new_loaded(miner: MinerData) -> Self {
        Self {
            state: DeviceDetailState::Loaded { miner },
        }
    }

    pub fn update_with_data(&mut self, result: Result<MinerData, FetchError>) {
        self.state = match result {
            Ok(miner) => DeviceDetailState::Loaded { miner },
            Err(error) => DeviceDetailState::Error(error.to_string()),
        };
    }

    pub fn view(&self) -> Element<'_, DeviceDetailMessage> {
        match &self.state {
            DeviceDetailState::Loading(ip) => {
                let content = column![
                    self.view_loading_header(ip),
                    container(
                        column![
                            theme::icons::icon_size(theme::icons::REFRESH, 64.0),
                            theme::typography::heading("Loading miner data..."),
                            theme::typography::body(format!("Fetching complete data from {}", ip)),
                        ]
                        .spacing(theme::spacing::MD)
                        .align_x(iced::Alignment::Center)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                ]
                .spacing(theme::spacing::LG)
                .padding(theme::padding::LG);

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }

            DeviceDetailState::Loaded { miner } => {
                let content = scrollable(
                    column![
                        self.view_header(miner),
                        // Hardware + Performance side by side
                        row![
                            self.view_hardware_section(miner),
                            self.view_performance_section(miner),
                        ]
                        .spacing(theme::spacing::SM),
                        // Hashboards full width
                        self.view_hashboards_section(miner),
                        // Cooling + Power side by side
                        row![
                            self.view_cooling_section(miner),
                            self.view_power_section(miner),
                        ]
                        .spacing(theme::spacing::SM),
                        // Pools full width
                        self.view_pools_section(miner),
                        if !miner.messages.is_empty() {
                            self.view_messages_section(miner)
                        } else {
                            column![].into()
                        },
                    ]
                    .spacing(theme::spacing::SM)
                    .padding(theme::padding::SM),
                );

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }

            DeviceDetailState::Error(error) => {
                let content = column![
                    self.view_error_header(),
                    container(
                        column![
                            theme::icons::icon_size(theme::icons::ERROR, 64.0),
                            theme::typography::heading("Failed to load miner data"),
                            theme::typography::body(error),
                        ]
                        .spacing(theme::spacing::MD)
                        .align_x(iced::Alignment::Center)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
                    .align_y(iced::alignment::Vertical::Center)
                ]
                .spacing(theme::spacing::LG)
                .padding(theme::padding::LG);

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }

    fn view_loading_header(&self, ip: &IpAddr) -> Element<'_, DeviceDetailMessage> {
        let back_button = secondary_button(
            "Back",
            Some(theme::icons::back().into()),
            Some(DeviceDetailMessage::Back),
        );

        container(
            row![
                back_button,
                Space::new().width(theme::spacing::SM),
                theme::typography::mono(format!("{}", ip)),
                Space::new().width(Length::Fill),
            ]
            .align_y(iced::Alignment::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::SM)
        .width(Length::Fill)
        .into()
    }

    fn view_error_header(&self) -> Element<'_, DeviceDetailMessage> {
        let title = theme::typography::title("Error Loading Device");
        let back_button = secondary_button(
            "Back",
            Some(theme::icons::back().into()),
            Some(DeviceDetailMessage::Back),
        );

        container(
            row![title, Space::new().width(Length::Fill), back_button]
                .align_y(iced::Alignment::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::MD)
        .width(Length::Fill)
        .into()
    }

    fn view_header(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let back_button = secondary_button(
            "Back",
            Some(theme::icons::back().into()),
            Some(DeviceDetailMessage::Back),
        );
        let browser_button = secondary_button(
            "Open Web UI",
            Some(theme::icons::network().into()),
            Some(DeviceDetailMessage::OpenInBrowser),
        );
        let restart_button = danger_button(
            "Restart",
            Some(theme::icons::refresh().into()),
            Some(DeviceDetailMessage::Restart),
        );

        container(
            row![
                back_button,
                Space::new().width(theme::spacing::SM),
                theme::typography::mono(format!("{}", miner.ip)),
                Space::new().width(Length::Fill),
                row![browser_button, restart_button].spacing(theme::spacing::XS)
            ]
            .align_y(iced::Alignment::Center),
        )
        .style(theme::containers::header)
        .padding(theme::padding::SM)
        .width(Length::Fill)
        .into()
    }

    fn view_hardware_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let info = &miner.device_info;

        let items = column![
            theme::typography::heading("Hardware Information"),
            self.info_row("Manufacturer", format!("{}", info.make)),
            self.info_row("Model", format!("{}", info.model)),
            self.info_row("Firmware", format!("{}", info.firmware)),
            self.info_row("Algorithm", format!("{}", info.algo)),
            self.info_row(
                "MAC Address",
                miner
                    .mac
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Hostname",
                miner.hostname.clone().unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Serial Number",
                miner
                    .serial_number
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Control Board",
                miner
                    .control_board_version
                    .as_ref()
                    .map(|cb| format!("{}", cb))
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Firmware Version",
                miner
                    .firmware_version
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Uptime",
                miner
                    .uptime
                    .map(|u| format_duration(u.as_secs()))
                    .unwrap_or_else(|| "N/A".to_string())
            ),
            self.info_row(
                "Fault Light",
                miner
                    .light_flashing
                    .map(|l| if l { "Flashing" } else { "Off" }.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            ),
        ]
        .spacing(theme::spacing::XS);

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_performance_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let hashrate_str = miner
            .hashrate
            .as_ref()
            .map(|hr| format!("{:.2}", hr))
            .unwrap_or_else(|| "N/A".to_string());

        let expected_hashrate_str = miner
            .expected_hashrate
            .as_ref()
            .map(|hr| format!("{:.2}", hr))
            .unwrap_or_else(|| "N/A".to_string());

        let hashrate_percentage = miner
            .hashrate
            .as_ref()
            .zip(miner.expected_hashrate.as_ref())
            .map(|(current, expected)| {
                let pct = (current.value / expected.value * 100.0) as u32;
                format!("{}%", pct)
            })
            .unwrap_or_else(|| "N/A".to_string());

        let efficiency_str = miner
            .efficiency
            .map(|eff| format!("{:.2} W/TH", eff))
            .unwrap_or_else(|| "N/A".to_string());

        let mining_status = if miner.is_mining {
            "Active"
        } else {
            "Inactive"
        };

        let items = column![
            theme::typography::heading("Performance"),
            self.info_row("Status", mining_status.to_string()),
            self.info_row("Hashrate", hashrate_str),
            self.info_row("Expected Hashrate", expected_hashrate_str),
            self.info_row("Efficiency", hashrate_percentage),
            self.info_row("Power Efficiency", efficiency_str),
        ]
        .spacing(theme::spacing::XS);

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_hashboards_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let board_count = format!(
            "{}/{}",
            miner.hashboards.len(),
            miner.expected_hashboards.unwrap_or(0)
        );

        let mut items = column![
            theme::typography::heading("Hashboards"),
            self.info_row("Detected", board_count),
        ]
        .spacing(theme::spacing::XS);

        for (idx, board) in miner.hashboards.iter().enumerate() {
            let board_label = format!("Board {}", board.position);

            let board_info = column![
                text(board_label).size(14),
                self.info_row(
                    "Working Chips",
                    board
                        .working_chips
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                ),
                self.info_row(
                    "Board Temp",
                    board
                        .board_temperature
                        .map(|t| format!("{:.1}°C", t.as_celsius()))
                        .unwrap_or_else(|| "N/A".to_string())
                ),
                self.info_row(
                    "Hashrate",
                    board
                        .hashrate
                        .as_ref()
                        .map(|hr| format!("{:.2}", hr))
                        .unwrap_or_else(|| "N/A".to_string())
                ),
            ]
            .spacing(theme::spacing::XS);

            let _ = idx; // silence unused warning

            items = items.push(
                container(board_info)
                    .padding(theme::padding::SM)
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(
                            theme::colors::BACKGROUND_ELEVATED,
                        )),
                        border: iced::Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: theme::colors::BORDER_SUBTLE,
                        },
                        ..container::Style::default()
                    })
                    .width(Length::Fill),
            );
        }

        let total_chips_str = format!(
            "{}/{}",
            miner.total_chips.unwrap_or(0),
            miner.expected_chips.unwrap_or(0)
        );

        items = items.push(self.info_row("Total Working Chips", total_chips_str));

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_cooling_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let mut items = column![
            theme::typography::heading("Cooling"),
            self.info_row(
                "Average Temperature",
                miner
                    .average_temperature
                    .map(|t| format!("{:.1}°C", t.as_celsius()))
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        ]
        .spacing(theme::spacing::XS);

        if let Some(fluid_temp) = miner.fluid_temperature {
            items = items.push(self.info_row(
                "Fluid Temperature",
                format!("{:.1}°C", fluid_temp.as_celsius()),
            ));
        }

        // Only show fans section if there are fans (skip for immersion/hydro)
        if !miner.fans.is_empty() {
            let fan_count = format!(
                "{}/{}",
                miner.fans.len(),
                miner.expected_fans.unwrap_or(miner.fans.len() as u8)
            );
            items = items.push(self.info_row("Fans", fan_count));

            for (idx, fan) in miner.fans.iter().enumerate() {
                let fan_label = format!("Fan {}", fan.position);
                let _ = idx; // silence unused warning

                items = items.push(
                    self.info_row(
                        fan_label,
                        fan.rpm
                            .map(|rpm| format!("{:.0} RPM", rpm.as_rpm()))
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                );
            }
        }

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_power_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let mut items = column![theme::typography::heading("Power"),].spacing(theme::spacing::XS);

        items = items.push(
            self.info_row(
                "Current Draw",
                miner
                    .wattage
                    .map(|w| format!("{:.0} W", w.as_watts()))
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        );

        items = items.push(
            self.info_row(
                "Power Limit",
                miner
                    .wattage_limit
                    .map(|w| format!("{:.0} W", w.as_watts()))
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        );

        items = items.push(
            self.info_row(
                "Efficiency",
                miner
                    .efficiency
                    .map(|eff| format!("{:.2} W/TH", eff))
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        );

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_pools_section(&self, miner: &MinerData) -> Element<'_, DeviceDetailMessage> {
        let mut items =
            column![theme::typography::heading("Mining Pools"),].spacing(theme::spacing::XS);

        for (idx, pool) in miner.pools.iter().enumerate() {
            let pool_info = column![
                text(format!("Pool {}", idx + 1)).size(14),
                self.info_row(
                    "URL",
                    pool.url
                        .as_ref()
                        .map(|u| u.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                ),
                self.info_row(
                    "User",
                    pool.user.clone().unwrap_or_else(|| "N/A".to_string())
                ),
                self.info_row(
                    "Status",
                    if pool.active.unwrap_or(false) {
                        "Active"
                    } else {
                        "Inactive"
                    }
                    .to_string()
                ),
            ]
            .spacing(theme::spacing::XS);

            items = items.push(
                container(pool_info)
                    .padding(theme::padding::SM)
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(
                            theme::colors::BACKGROUND_ELEVATED,
                        )),
                        border: iced::Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: theme::colors::BORDER_SUBTLE,
                        },
                        ..container::Style::default()
                    })
                    .width(Length::Fill),
            );
        }

        if miner.pools.is_empty() {
            items = items.push(text("No pools configured"));
        }

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn view_messages_section<'a>(&self, miner: &'a MinerData) -> Element<'a, DeviceDetailMessage> {
        let mut items =
            column![theme::typography::heading("Messages & Alerts"),].spacing(theme::spacing::XS);

        for msg in &miner.messages {
            items = items.push(
                row![theme::icons::warning(), text(&msg.message),].spacing(theme::spacing::XS),
            );
        }

        container(items)
            .padding(theme::padding::SM)
            .style(theme::containers::card)
            .width(Length::Fill)
            .into()
    }

    fn info_row(
        &self,
        label: impl ToString,
        value: impl ToString,
    ) -> Element<'_, DeviceDetailMessage> {
        row![
            text(format!("{}:", label.to_string()))
                .width(Length::FillPortion(1))
                .style(|_theme: &iced::Theme| {
                    text::Style {
                        color: Some(theme::colors::TEXT_SECONDARY),
                    }
                }),
            text(value.to_string()).width(Length::FillPortion(2)),
        ]
        .spacing(theme::spacing::XS)
        .into()
    }
}
