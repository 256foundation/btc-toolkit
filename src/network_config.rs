use crate::config::{AppConfig, ScanGroup};
use crate::network::scanner::ScanConfig;
use crate::theme::{self, BtcTheme};
use asic_rs::data::device::{MinerFirmware, MinerMake};
use iced::widget::{Space, button, checkbox, column, container, row, scrollable, text, text_input};
use iced::{Element, Length};

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    app_config: AppConfig,
    // UI state for group editing
    editing_group: Option<EditingGroup>,
    // UI state for filters (applied to currently editing group)
    filter_antminers: bool,
    filter_whatsminers: bool,
    filter_avalonminers: bool,
    filter_braiins_os: bool,
    filter_stock_firmware: bool,
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
    ToggleAntMiners(bool),
    ToggleWhatsMiners(bool),
    ToggleAvalonMiners(bool),
    ToggleBraiinsOS(bool),
    ToggleStockFirmware(bool),
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            app_config: AppConfig::default(),
            editing_group: None,
            filter_antminers: false,
            filter_whatsminers: false,
            filter_avalonminers: false,
            filter_braiins_os: false,
            filter_stock_firmware: false,
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
            NetworkConfigMessage::ToggleAntMiners(enabled) => {
                self.filter_antminers = enabled;
            }
            NetworkConfigMessage::ToggleWhatsMiners(enabled) => {
                self.filter_whatsminers = enabled;
            }
            NetworkConfigMessage::ToggleAvalonMiners(enabled) => {
                self.filter_avalonminers = enabled;
            }
            NetworkConfigMessage::ToggleBraiinsOS(enabled) => {
                self.filter_braiins_os = enabled;
            }
            NetworkConfigMessage::ToggleStockFirmware(enabled) => {
                self.filter_stock_firmware = enabled;
            }
            // Close and Save are handled in main app
            NetworkConfigMessage::Close | NetworkConfigMessage::Save => {}
        }
    }

    fn reset_filters(&mut self) {
        self.filter_antminers = false;
        self.filter_whatsminers = false;
        self.filter_avalonminers = false;
        self.filter_braiins_os = false;
        self.filter_stock_firmware = false;
    }

    fn load_filters_from_group(&mut self, scan_config: &ScanConfig) {
        self.reset_filters();

        if let Some(ref makes) = scan_config.search_makes {
            for make in makes {
                match make {
                    MinerMake::AntMiner => self.filter_antminers = true,
                    MinerMake::WhatsMiner => self.filter_whatsminers = true,
                    MinerMake::AvalonMiner => self.filter_avalonminers = true,
                    _ => {}
                }
            }
        }

        if let Some(ref firmwares) = scan_config.search_firmwares {
            for firmware in firmwares {
                match firmware {
                    MinerFirmware::BraiinsOS => self.filter_braiins_os = true,
                    MinerFirmware::Stock => self.filter_stock_firmware = true,
                    _ => {}
                }
            }
        }
    }

    fn build_scan_config(&self) -> ScanConfig {
        let mut makes = Vec::new();
        let mut firmwares = Vec::new();

        if self.filter_antminers {
            makes.push(MinerMake::AntMiner);
        }
        if self.filter_whatsminers {
            makes.push(MinerMake::WhatsMiner);
        }
        if self.filter_avalonminers {
            makes.push(MinerMake::AvalonMiner);
        }

        if self.filter_braiins_os {
            firmwares.push(MinerFirmware::BraiinsOS);
        }
        if self.filter_stock_firmware {
            firmwares.push(MinerFirmware::Stock);
        }

        ScanConfig {
            search_makes: if makes.is_empty() { None } else { Some(makes) },
            search_firmwares: if firmwares.is_empty() {
                None
            } else {
                Some(firmwares)
            },
        }
    }

    pub fn view(&self) -> Element<NetworkConfigMessage> {
        if let Some(ref editing) = self.editing_group {
            // Show group editing form
            self.view_group_editor(editing)
        } else {
            // Show groups list
            self.view_groups_list()
        }
    }

    fn view_groups_list(&self) -> Element<NetworkConfigMessage> {
        let _theme = BtcTheme::default();

        // Header section
        let header = container(
            row![
                column![
                    theme::typography::title("⚙️ Network Configuration"),
                    theme::typography::small("Configure scan groups for ASIC miner discovery")
                ]
                .spacing(theme::layout::SPACING_XS),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![
                        text("➕").size(16),
                        theme::typography::body("Add New Group")
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(iced::widget::button::primary)
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
                    text("📁").size(48),
                    theme::typography::heading("No Scan Groups Configured"),
                    theme::typography::body(
                        "Create your first scan group to start discovering miners"
                    ),
                    Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
                    button(
                        row![
                            text("➕").size(16),
                            theme::typography::body("Create First Group")
                        ]
                        .spacing(theme::layout::SPACING_SM)
                        .align_y(iced::alignment::Vertical::Center)
                    )
                    .style(iced::widget::button::primary)
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
                                row![text("✏️").size(14), theme::typography::small("Edit")]
                                    .spacing(theme::layout::SPACING_XS)
                                    .align_y(iced::alignment::Vertical::Center)
                            )
                            .style(iced::widget::button::secondary)
                            .padding(theme::layout::PADDING_SM)
                            .width(theme::layout::BUTTON_WIDTH)
                            .on_press(NetworkConfigMessage::EditGroup(group.name.clone())),
                            button(
                                row![text("🗑️").size(14), theme::typography::small("Delete")]
                                    .spacing(theme::layout::SPACING_XS)
                                    .align_y(iced::alignment::Vertical::Center)
                            )
                            .style(iced::widget::button::danger)
                            .padding(theme::layout::PADDING_SM)
                            .width(theme::layout::BUTTON_WIDTH)
                            .on_press(NetworkConfigMessage::DeleteGroup(group.name.clone()))
                        ]
                        .spacing(theme::layout::SPACING_SM)
                    ]
                    .spacing(theme::layout::SPACING_MD)
                    .align_y(iced::alignment::Vertical::Center),
                )
                .style(iced::widget::container::rounded_box)
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
                    row![text("❌").size(16), theme::typography::body("Cancel")]
                        .spacing(theme::layout::SPACING_SM)
                        .align_y(iced::alignment::Vertical::Center)
                )
                .style(iced::widget::button::secondary)
                .padding(theme::layout::PADDING_SM)
                .width(theme::layout::BUTTON_WIDTH)
                .on_press(NetworkConfigMessage::Close),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![
                        text("💾").size(16),
                        theme::typography::body("Save Configuration")
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(iced::widget::button::primary)
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

    fn view_group_editor(&self, editing: &EditingGroup) -> Element<NetworkConfigMessage> {
        let _theme = BtcTheme::default();

        let is_editing = editing.original_name.is_some();
        let title_text = if is_editing {
            "✏️ Edit Scan Group"
        } else {
            "➕ Add New Scan Group"
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
                theme::typography::heading("🔧 Basic Configuration"),
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),
                // Group name input
                row![
                    theme::typography::body("Group Name:").width(theme::layout::LABEL_WIDTH),
                    text_input("e.g. Farm A", &editing.name)
                        .style(theme::text_input_styles::default)
                        .on_input(NetworkConfigMessage::SetGroupName)
                        .padding(theme::layout::PADDING_SM)
                        .width(theme::layout::INPUT_WIDTH)
                ]
                .spacing(theme::layout::SPACING_MD)
                .align_y(iced::alignment::Vertical::Center),
                // IP range input
                row![
                    theme::typography::body("IP Range:").width(theme::layout::LABEL_WIDTH),
                    text_input("e.g. 192.168.1.0/24", &editing.network_range)
                        .style(theme::text_input_styles::default)
                        .on_input(NetworkConfigMessage::SetGroupNetworkRange)
                        .padding(theme::layout::PADDING_SM)
                        .width(theme::layout::INPUT_WIDTH)
                ]
                .spacing(theme::layout::SPACING_MD)
                .align_y(iced::alignment::Vertical::Center),
                // Enabled checkbox
                row![
                    checkbox("Enable this group for scanning", editing.enabled)
                        .style(theme::checkbox_styles::default)
                        .on_toggle(NetworkConfigMessage::SetGroupEnabled)
                ]
                .spacing(theme::layout::SPACING_MD),
                container(theme::typography::small(
                    "💡 Supports CIDR notation (192.168.1.0/24) or IP ranges (192.168.1.1-100)"
                ))
                .style(iced::widget::container::rounded_box)
                .padding(theme::layout::PADDING_SM)
                .width(Length::Fill)
            ]
            .spacing(theme::layout::SPACING_MD),
        )
        .style(theme::container_styles::card)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        // Filter configuration
        let filter_config = container(
            column![
                theme::typography::heading("🔍 Miner Filters"),
                theme::typography::small("Configure which types of miners to discover (leave all unchecked to find all types)"),
                Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_MD)),

                row![
                    // Miner make filters
                    container(
                        column![
                            theme::typography::body("Miner Manufacturers:"),
                            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),

                            checkbox("🐜 AntMiner (Bitmain)", self.filter_antminers)
                .style(theme::checkbox_styles::default)
                                .on_toggle(NetworkConfigMessage::ToggleAntMiners),
                            checkbox("❓ WhatsMiner (MicroBT)", self.filter_whatsminers)
                .style(theme::checkbox_styles::default)
                                .on_toggle(NetworkConfigMessage::ToggleWhatsMiners),
                            checkbox("🏔️ AvalonMiner (Canaan)", self.filter_avalonminers)
                .style(theme::checkbox_styles::default)
                                .on_toggle(NetworkConfigMessage::ToggleAvalonMiners),
                        ]
                        .spacing(theme::layout::SPACING_SM)
                    )
                    .width(Length::FillPortion(1)),

                    Space::new(Length::Fixed(theme::layout::SPACING_LG), Length::Fixed(0.0)),

                    // Firmware filters
                    container(
                        column![
                            theme::typography::body("Firmware Types:"),
                            Space::new(Length::Fixed(0.0), Length::Fixed(theme::layout::SPACING_SM)),

                            checkbox("🧠 Braiins OS (Custom)", self.filter_braiins_os)
                .style(theme::checkbox_styles::default)
                                .on_toggle(NetworkConfigMessage::ToggleBraiinsOS),
                            checkbox("🏭 Stock Firmware (Factory)", self.filter_stock_firmware)
                .style(theme::checkbox_styles::default)
                                .on_toggle(NetworkConfigMessage::ToggleStockFirmware),
                        ]
                        .spacing(theme::layout::SPACING_SM)
                    )
                    .width(Length::FillPortion(1))
                ]
                .spacing(theme::layout::SPACING_LG)
            ]
            .spacing(theme::layout::SPACING_SM)
        )
        .style(theme::container_styles::card)
        .padding(theme::layout::PADDING_MD)
        .width(Length::Fill);

        // Action buttons
        let action_buttons = container(
            row![
                button(
                    row![text("❌").size(16), theme::typography::body("Cancel")]
                        .spacing(theme::layout::SPACING_SM)
                        .align_y(iced::alignment::Vertical::Center)
                )
                .style(iced::widget::button::secondary)
                .padding(theme::layout::PADDING_SM)
                .width(theme::layout::BUTTON_WIDTH)
                .on_press(NetworkConfigMessage::CancelGroupEdit),
                Space::new(Length::Fill, Length::Fixed(0.0)),
                button(
                    row![
                        text(if is_editing { "💾" } else { "➕" }).size(16),
                        theme::typography::body(if is_editing {
                            "Save Changes"
                        } else {
                            "Create Group"
                        })
                    ]
                    .spacing(theme::layout::SPACING_SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .style(iced::widget::button::primary)
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
                .max_width(900) // Limit max width for better form layout
                .center_x(Length::Fill);

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
