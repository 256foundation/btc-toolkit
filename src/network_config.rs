use crate::config::{AppConfig, ScanGroup};
use crate::network::scanner::ScanConfig;
use crate::theme::{self, BtcTheme};
use asic_rs::data::device::{MinerFirmware, MinerMake};
use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    app_config: AppConfig,
    // UI state for group editing
    editing_group: Option<EditingGroup>,
    // UI state for filters (applied to currently editing group)
    search_firmwares: HashSet<MinerFirmware>,
    search_makes: HashSet<MinerMake>,
}

#[derive(Clone, Debug)]
struct EditingGroup {
    original_name: Option<String>, // None if creating new group
    name: String,
    network_range: String,
    enabled: bool,
}

#[derive(Debug, Clone)]
pub enum NetworkConfigMessage {
    Close,
    Save,
    // Group management
    AddNewGroup,
    EditGroup(String),
    DeleteGroup(String),
    ToggleGroupEnabled(String, bool),
    // Group editing
    SetGroupName(String),
    SetGroupNetworkRange(String),
    SetGroupEnabled(bool),
    SaveGroup,
    CancelGroupEdit,
    // Filter toggles (for currently editing group)
    ToggleFirmware(MinerFirmware, bool),
    ToggleMake(MinerMake, bool),
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            app_config: AppConfig::default(),
            editing_group: None,
            search_makes: HashSet::new(),
            search_firmwares: HashSet::new(),
        }
    }

    pub fn set_app_config(&mut self, config: AppConfig) {
        self.app_config = config;
    }

    pub fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }

    pub fn update(&mut self, msg: NetworkConfigMessage) {
        match msg {
            NetworkConfigMessage::AddNewGroup => {
                self.editing_group = Some(EditingGroup {
                    original_name: None,
                    name: "New Group".to_string(),
                    network_range: "192.168.1.0/24".to_string(),
                    enabled: true,
                });
                self.reset_filters();
            }
            NetworkConfigMessage::EditGroup(name) => {
                if let Some(group) = self.app_config.get_group(&name).cloned() {
                    self.editing_group = Some(EditingGroup {
                        original_name: Some(name.clone()),
                        name: group.name.clone(),
                        network_range: group.network_range.clone(),
                        enabled: group.enabled,
                    });
                    self.load_filters_from_group(&group.scan_config);
                }
            }
            NetworkConfigMessage::DeleteGroup(name) => {
                self.app_config.remove_scan_group(&name);
            }
            NetworkConfigMessage::ToggleGroupEnabled(name, enabled) => {
                if let Some(group) = self.app_config.get_group_mut(&name) {
                    group.enabled = enabled;
                }
            }
            NetworkConfigMessage::SetGroupName(name) => {
                if let Some(ref mut editing) = self.editing_group {
                    editing.name = name;
                }
            }
            NetworkConfigMessage::SetGroupNetworkRange(range) => {
                if let Some(ref mut editing) = self.editing_group {
                    editing.network_range = range;
                }
            }
            NetworkConfigMessage::SetGroupEnabled(enabled) => {
                if let Some(ref mut editing) = self.editing_group {
                    editing.enabled = enabled;
                }
            }
            NetworkConfigMessage::SaveGroup => {
                if let Some(editing) = &self.editing_group {
                    let scan_config = self.build_scan_config();

                    let new_group = ScanGroup {
                        name: editing.name.clone(),
                        network_range: editing.network_range.clone(),
                        scan_config,
                        enabled: editing.enabled,
                    };

                    if let Some(ref original_name) = editing.original_name {
                        // Editing existing group
                        self.app_config.update_scan_group(original_name, new_group);
                    } else {
                        // Adding new group
                        self.app_config.add_scan_group(new_group);
                    }

                    self.editing_group = None;
                }
            }
            NetworkConfigMessage::CancelGroupEdit => {
                self.editing_group = None;
                self.reset_filters();
            }
            NetworkConfigMessage::ToggleFirmware(firmware, enable) => {
                if enable {
                    self.search_firmwares.insert(firmware);
                } else {
                    self.search_firmwares.remove(&firmware);
                }
            }
            NetworkConfigMessage::ToggleMake(make, enable) => {
                if enable {
                    self.search_makes.insert(make);
                } else {
                    self.search_makes.remove(&make);
                }
            }
            // Close and Save are handled in main app
            NetworkConfigMessage::Close | NetworkConfigMessage::Save => {}
        }
    }

    fn reset_filters(&mut self) {
        self.search_firmwares.clear();
        self.search_makes.clear();
    }

    fn load_filters_from_group(&mut self, scan_config: &ScanConfig) {
        self.reset_filters();

        if let Some(ref makes) = scan_config.search_makes {
            makes.iter().for_each(|make| {
                self.search_makes.insert(make.clone());
            });
        }

        if let Some(ref firmwares) = scan_config.search_firmwares {
            firmwares.iter().for_each(|firmware| {
                self.search_firmwares.insert(firmware.clone());
            });
        }
    }

    fn build_scan_config(&self) -> ScanConfig {
        let makes = Vec::from_iter(self.search_makes.iter().cloned());
        let firmwares = Vec::from_iter(self.search_firmwares.iter().cloned());

        ScanConfig {
            search_makes: if makes.is_empty() { None } else { Some(makes) },
            search_firmwares: if firmwares.is_empty() {
                None
            } else {
                Some(firmwares)
            },
        }
    }

    pub fn view(&self) -> Element<'_, NetworkConfigMessage> {
        if let Some(ref editing) = self.editing_group {
            // Show group editing form
            self.view_group_editor(editing)
        } else {
            // Show groups list
            self.view_groups_list()
        }
    }

    fn view_groups_list(&self) -> Element<'_, NetworkConfigMessage> {
        let _theme = BtcTheme::default();

        // Header section
        let header = container(
            row![
                column![
                    theme::typography::title("Network Configuration"),
                    theme::typography::small("Configure scan groups for ASIC miner discovery")
                ]
                .spacing(theme::layout::SPACING_XS),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![text("+").size(16), theme::typography::body("Add New Group")]
                        .spacing(theme::layout::SPACING_SM)
                        .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::primary)
                .padding(theme::layout::PADDING_SM)
                .on_press(NetworkConfigMessage::AddNewGroup)
            ]
            .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::container_styles::header)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        // Groups list section
        let groups_content = if self.app_config.scan_groups.is_empty() {
            container(
                column![
                    theme::typography::heading("No Scan Groups Configured"),
                    theme::typography::body(
                        "Create your first scan group to start discovering miners"
                    ),
                    Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
                    button(
                        row![
                            text("+").size(16),
                            theme::typography::body("Create First Group")
                        ]
                        .spacing(theme::layout::SPACING_SM)
                        .align_y(iced::alignment::Vertical::Center)
                    )
                    .style(button::primary)
                    .padding(theme::layout::PADDING_MD)
                    .on_press(NetworkConfigMessage::AddNewGroup)
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(theme::layout::SPACING_MD),
            )
            .padding(theme::layout::PADDING_XL)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        } else {
            let mut groups_list = column![].spacing(theme::layout::SPACING_MD);

            for group in &self.app_config.scan_groups {
                let enabled_checkbox = checkbox("", group.enabled)
                    .style(theme::checkbox_styles::default)
                    .on_toggle(move |enabled| {
                        NetworkConfigMessage::ToggleGroupEnabled(group.name.clone(), enabled)
                    });

                let filters_summary = self.format_filters_summary(&group.scan_config);

                let group_card = container(
                    row![
                        enabled_checkbox,
                        column![
                            row![
                                theme::typography::heading(&group.name),
                                Space::new(Length::Fill, Length::Fixed(0.0)),
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
                            .align_y(iced::alignment::Vertical::Center),
                            theme::typography::mono(&group.network_range),
                            theme::typography::small(filters_summary)
                        ]
                        .spacing(theme::layout::SPACING_XS)
                        .width(Length::Fill),
                        column![
                            button(
                                row![theme::typography::small("Edit")]
                                    .spacing(theme::layout::SPACING_XS)
                                    .align_y(iced::alignment::Vertical::Center)
                            )
                            .style(button::secondary)
                            .padding(theme::layout::PADDING_SM)
                            .width(theme::layout::BUTTON_WIDTH)
                            .on_press(NetworkConfigMessage::EditGroup(group.name.clone())),
                            button(
                                row![theme::typography::small("Delete")]
                                    .spacing(theme::layout::SPACING_XS)
                                    .align_y(iced::alignment::Vertical::Center)
                            )
                            .style(button::danger)
                            .padding(theme::layout::PADDING_SM)
                            .width(theme::layout::BUTTON_WIDTH)
                            .on_press(NetworkConfigMessage::DeleteGroup(group.name.clone()))
                        ]
                        .spacing(theme::layout::SPACING_SM)
                    ]
                    .spacing(theme::layout::SPACING_MD)
                    .align_y(iced::alignment::Vertical::Center),
                )
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::Fill);

                groups_list = groups_list.push(group_card);
            }

            container(scrollable(groups_list).height(Length::Fill))
                .padding(theme::layout::PADDING_MD)
        };

        // Action buttons
        let action_buttons = container(
            row![
                button(
                    row![
                        theme::typography::body("Cancel")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::secondary)
                .padding(theme::layout::PADDING_SM)
                .on_press(NetworkConfigMessage::Close),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![
                        theme::typography::body("Save Configuration")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::primary)
                .padding(theme::layout::PADDING_SM)
                .on_press(NetworkConfigMessage::Save)
            ]
            .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::container_styles::header)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        let content = column![header, groups_content, action_buttons].spacing(0); // No spacing since containers have their own padding

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_group_editor(&self, editing: &EditingGroup) -> Element<'_, NetworkConfigMessage> {
        let _theme = BtcTheme::default();

        let is_editing = editing.original_name.is_some();
        let title_text = if is_editing {
            "Edit Scan Group"
        } else {
            "Add New Scan Group"
        };

        // Header section
        let header = container(row![
            column![
                theme::typography::title(title_text),
                theme::typography::small(if is_editing {
                    "Modify scan group configuration and filters"
                } else {
                    "Create a new network range for ASIC miner discovery"
                })
            ]
            .spacing(theme::layout::SPACING_XS),
            Space::new(Length::Fill, Length::Fixed(0.0))
        ])
        .style(theme::container_styles::header)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        // Basic configuration form
        let basic_config = container(
            column![
                theme::typography::heading("Basic Configuration"),
                // Group name input
                container(
                    row![
                        theme::typography::body("Group Name:").width(theme::layout::LABEL_WIDTH),
                        text_input("e.g. Farm A", &editing.name)
                            .style(theme::text_input_styles::default)
                            .on_input(NetworkConfigMessage::SetGroupName)
                            .padding(theme::layout::PADDING_SM)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_MD)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::Fill),
                // IP range input
                container(column![
                    row![
                        theme::typography::body("IP Range:").width(theme::layout::LABEL_WIDTH),
                        text_input("e.g. 192.168.1.0/24", &editing.network_range)
                            .style(theme::text_input_styles::default)
                            .on_input(NetworkConfigMessage::SetGroupNetworkRange)
                            .padding(theme::layout::PADDING_SM)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_MD)
                    .align_y(iced::alignment::Vertical::Center),
                    Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
                    theme::typography::small(
                        "Supports CIDR notation (192.168.1.0/24) or IP ranges (192.168.1.1-100)"
                    )
                ])
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::Fill),
                // Enabled checkbox
                container(
                    row![
                        checkbox("Enable this group for scanning", editing.enabled)
                            .style(theme::checkbox_styles::default)
                            .on_toggle(NetworkConfigMessage::SetGroupEnabled)
                    ]
                    .spacing(theme::layout::SPACING_MD),
                )
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)
                .width(Length::Fill),
            ]
            .spacing(theme::layout::SPACING_MD),
        )
        .style(theme::container_styles::card)
        .padding(theme::layout::PADDING_XL)
        .width(Length::Fill);

        // Filter configuration
        let filter_config = container(
            column![
                theme::typography::heading("Miner Filters"),
                theme::typography::small("Configure which types of miners to discover (leave all unchecked to find all types)"),
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),

                container(
                    row![
                        // Miner make filters
                        container(
                            column![
                                theme::typography::body("Miner Manufacturers:"),
                                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),

                                checkbox("AntMiner (Bitmain)", self.search_makes.contains(&MinerMake::AntMiner))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::AntMiner, value)),
                                checkbox("WhatsMiner (MicroBT)", self.search_makes.contains(&MinerMake::WhatsMiner))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::WhatsMiner, value)),
                                checkbox("AvalonMiner (Canaan)", self.search_makes.contains(&MinerMake::AvalonMiner))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::AvalonMiner, value)),
                                checkbox("BitAxe", self.search_makes.contains(&MinerMake::BitAxe))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::BitAxe, value)),
                                checkbox("ePIC", self.search_makes.contains(&MinerMake::EPic))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::EPic, value)),
                                checkbox("Braiins", self.search_makes.contains(&MinerMake::Braiins))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleMake(MinerMake::Braiins, value)),
                            ]
                            .spacing(theme::layout::SPACING_SM)
                        )
                        .width(Length::FillPortion(1)),

                        Space::new(Length::Fixed(theme::layout::SPACING_MD), Length::Fixed(0.0)),

                        // Firmware filters
                        container(
                            column![
                                theme::typography::body("Firmware Types:"),
                                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),

                                checkbox("Braiins OS", self.search_firmwares.contains(&MinerFirmware::BraiinsOS))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleFirmware(MinerFirmware::BraiinsOS, value)),
                                checkbox("ePIC UMC", self.search_firmwares.contains(&MinerFirmware::EPic))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleFirmware(MinerFirmware::EPic, value)),
                                checkbox("Luxor OS", self.search_firmwares.contains(&MinerFirmware::LuxOS))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleFirmware(MinerFirmware::LuxOS, value)),
                                checkbox("VNish", self.search_firmwares.contains(&MinerFirmware::VNish))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleFirmware(MinerFirmware::VNish, value)),
                                checkbox("Mara FW", self.search_firmwares.contains(&MinerFirmware::Marathon))
                                    .style(theme::checkbox_styles::default)
                                    .on_toggle(|value| NetworkConfigMessage::ToggleFirmware(MinerFirmware::Marathon, value)),
                            ]
                        .spacing(theme::layout::SPACING_SM)
                        )
                        .width(Length::FillPortion(1)),
                    ]
                    .spacing(theme::layout::SPACING_LG)
                )
                .style(theme::container_styles::card)
                .padding(theme::layout::PADDING_MD)

            ]
            .spacing(theme::layout::SPACING_SM)
        )
        .style(theme::container_styles::card)
        .padding(theme::layout::PADDING_XL)
        .width(Length::Fill);

        // Action buttons
        let action_buttons = container(
            row![
                button(
                    row![
                        theme::typography::body("Cancel")
                            .align_x(iced::alignment::Horizontal::Center)
                            .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::secondary)
                .padding(theme::layout::PADDING_SM)
                .on_press(NetworkConfigMessage::CancelGroupEdit),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![
                        theme::typography::body(if is_editing {
                            "Save Changes"
                        } else {
                            "Create Group"
                        })
                        .align_x(iced::alignment::Horizontal::Center)
                        .width(Length::Fill)
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(button::primary)
                .padding(theme::layout::PADDING_SM)
                .on_press(NetworkConfigMessage::SaveGroup)
            ]
            .align_y(iced::alignment::Vertical::Center),
        )
        .style(theme::container_styles::header)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        // Main content with side margins for better readability
        let main_content =
            container(column![basic_config, filter_config].spacing(theme::layout::SPACING_LG))
                .width(Length::Fill)
                .center_x(Length::Fill)
                .padding(theme::layout::PADDING_MD);

        let content = column![
            header,
            scrollable(main_content).height(Length::Fill),
            action_buttons
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn format_filters_summary(&self, scan_config: &ScanConfig) -> String {
        let mut parts = Vec::new();

        if let Some(ref makes) = scan_config.search_makes {
            if !makes.is_empty() {
                let make_names: Vec<String> = makes.iter().map(|m| format!("{m:?}")).collect();
                parts.push(format!("Makes: {}", make_names.join(", ")));
            }
        }

        if let Some(ref firmwares) = scan_config.search_firmwares {
            if !firmwares.is_empty() {
                let firmware_names: Vec<String> =
                    firmwares.iter().map(|f| format!("{f:?}")).collect();
                parts.push(format!("Firmware: {}", firmware_names.join(", ")));
            }
        }

        if parts.is_empty() {
            "No filters (scan all)".to_string()
        } else {
            parts.join(" | ")
        }
    }
}
