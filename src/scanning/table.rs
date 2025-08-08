use crate::scanning::ScanningMessage;
use crate::theme;
use asic_rs::data::miner::MinerData;
use iced::widget::{button, column, container, row, scrollable};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub(crate) struct ScanningTable {
    group: String,
    miners: Vec<MinerData>,
}
impl ScanningTable {
    pub fn new(group: String, miners: Vec<MinerData>) -> Self {
        Self { group, miners }
    }

    pub fn push(&mut self, miner: MinerData) {
        self.miners.push(miner);
    }

    pub fn view(&self) -> Element<ScanningMessage> {
        let table_header = container(
            row![
                    theme::typography::small("IP Address")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(3)),
                    theme::typography::small("Model")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(3)),
                    theme::typography::small("Make")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(2)),
                    theme::typography::small("Firmware")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(2)),
                    theme::typography::small("Firmware Version")
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::FillPortion(2)),
                ]
                .spacing(theme::spacing::SM),
        )
            .style(theme::containers::header)
            .padding(theme::padding::SM)
            .width(Length::Fill);

        let mut miners_list = iced::widget::column![]
            .spacing(theme::spacing::XS)
            .padding(theme::padding::SCROLLABLE);

        let mut sorted_miners = self.miners.clone();
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
                        .on_press(ScanningMessage::OpenIpInBrowser(miner_ip)),
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
                        theme::typography::body(format!(
                            "{}",
                            miner.firmware_version.unwrap_or("-".into())
                        ))
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
        };
        column![row![table_header], row![scrollable(miners_list)]].into()
    }
}