use crate::theme;
use iced::widget::{button, row, text};
use iced::{Element, alignment};

pub fn create_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<&'a str>,
    style: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    let content = if let Some(icon) = icon {
        Element::from(
            row![text(icon), text(label)]
                .spacing(theme::spacing::XS)
                .align_y(alignment::Vertical::Center),
        )
    } else {
        Element::from(text(label))
    };

    let mut btn = button(content).style(style).padding(theme::padding::SM);

    if let Some(msg) = message {
        btn = btn.on_press(msg);
    }

    btn
}

pub fn primary_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<&'a str>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::primary, message)
}

pub fn secondary_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<&'a str>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::secondary, message)
}

pub fn danger_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<&'a str>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::danger, message)
}

pub fn calculate_progress(completed: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        (completed as f32 / total as f32).clamp(0.0, 1.0)
    }
}

pub fn format_duration(seconds: u64) -> String {
    match seconds {
        0..=59 => format!("{}s", seconds),
        60..=3599 => format!("{}m {}s", seconds / 60, seconds % 60),
        _ => format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60),
    }
}
