use super::colors;
use iced::widget::button;
use iced::{Background, Border, Color, Theme};

/// Table row button style - card background with hover highlight
pub fn table_row(_theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Active => Some(Background::Color(colors::BACKGROUND_CARD)),
        button::Status::Hovered => Some(Background::Color(colors::BACKGROUND_ELEVATED)),
        button::Status::Pressed => Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.12))),
        button::Status::Disabled => Some(Background::Color(colors::BACKGROUND_CARD)),
    };

    button::Style {
        background,
        text_color: colors::TEXT_PRIMARY,
        border: Border {
            radius: 6.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: iced::Shadow::default(),
        snap: false,
    }
}
