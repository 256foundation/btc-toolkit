use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Length};

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub ip_range: String,
}

#[derive(Debug, Clone)]
pub enum NetworkConfigMessage {
    Open,
    Close,
    SetIpRange(String),
    Save,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            ip_range: String::from("192.168.1.0/24"),
        }
    }

    pub fn update(&mut self, msg: NetworkConfigMessage) {
        match msg {
            NetworkConfigMessage::SetIpRange(ip_range) => {
                self.ip_range = ip_range;
                println!("Set IP range to {}", self.ip_range);
            }
            // We handle Open/Close events in the main app
            NetworkConfigMessage::Open
            | NetworkConfigMessage::Close
            | NetworkConfigMessage::Save => {}
        }
    }

    pub fn view(&self) -> Element<NetworkConfigMessage> {
        let title = text("Network Configuration").size(24);

        let input_row = row![
            text("IP Range:").width(Length::Fixed(100.0)),
            text_input("e.g. 192.168.1.0/24", &self.ip_range)
                .on_input(NetworkConfigMessage::SetIpRange)
                .padding(8)
        ]
        .spacing(10);

        let buttons = row![
            button(text("Cancel"))
                .padding(10)
                .on_press(NetworkConfigMessage::Close),
            button(text("Save"))
                .padding(10)
                .on_press(NetworkConfigMessage::Save)
        ]
        .spacing(10);

        let content = column![title, input_row, buttons]
            .spacing(20)
            .padding(20)
            .max_width(400);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
