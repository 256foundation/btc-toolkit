use crate::scanning::message::ScanningMessage;
use crate::scanning::table::ScanningTable;
use crate::theme;
use asic_rs::data::miner::MinerData;
use iced::widget::{column, container, row, text, Column, Space};
use iced::Length;
use iced_aw::{TabBar, TabLabel};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ScanningTabs {
    active_tab: String,
    tabs: HashMap<String, Vec<MinerData>>,
    is_scanning: bool,
    tab_tables: HashMap<String, ScanningTable>,
}

impl ScanningTabs {
    pub fn new(active_tab: String, tabs: HashMap<String, Vec<MinerData>>) -> Self {
        let mut tables = HashMap::new();
        for (group, data) in &tabs {
            tables.insert(group.clone(), ScanningTable::new(
                group.clone(),
                data.clone(),
            ));
        }

        ScanningTabs {
            active_tab,
            tabs,
            is_scanning: true,
            tab_tables: tables,
        }
    }
    pub fn update(&mut self, message: ScanningMessage) {
        match message {
            ScanningMessage::MinerFound { group_name, miner } => {
                if !self.tabs.contains_key(group_name.as_str()) {
                    self.tabs.insert(group_name.to_string(), vec![]);
                }
                self.tabs.get_mut(group_name.as_str()).unwrap().push(miner.clone());
                self.tab_tables.get_mut(group_name.as_str()).unwrap().push(miner.clone());
            }
            ScanningMessage::GroupTabSelected(group_name) => {
                self.active_tab = group_name;
            }
            ScanningMessage::AllScansCompleted => {
                self.is_scanning = false;
            }
            ScanningMessage::StopScan => {
                self.is_scanning = false;
            }
            _ => {}
        }
    }
    pub fn view(&self) -> Column<'_, ScanningMessage> {
        let tab_view = column![
            self.tabs
            .iter()
            .fold(
                TabBar::new(ScanningMessage::GroupTabSelected),
                |tab_bar, (group, _)| {
                    // manually create a new index for the new tab
                    // starting from 0, when there is no tab created yet
                    tab_bar.push(group.clone(), TabLabel::Text(group.to_owned()))
                },
            )
            .set_active_tab(&self.active_tab)
            .tab_width(Length::Shrink)
            .spacing(theme::spacing::SM)
            .padding(theme::padding::SM)
        ];

        let inner_tab = if let Some(data) = self.tabs.get(&self.active_tab) {
            let group_header = container(
                row![
                    column![
                        theme::typography::heading(format!("{}", &self.active_tab)),
                        theme::typography::small(format!("{} miners discovered", data.len()))
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

            let group_content = column![
                self.tab_tables[&self.active_tab].view()
            ];

            column![group_header, group_content]
        } else {
            column![theme::typography::body("No groups found")]
        };

        column![tab_view, inner_tab]
    }
}
