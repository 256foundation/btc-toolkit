use crate::theme;
use iced::widget::{button, row, text};
use iced::{Element, alignment};

pub fn create_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<Element<'a, Message>>,
    style: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    let content = if let Some(icon) = icon {
        Element::from(
            row![icon, text(label)]
                .spacing(theme::spacing::SM)
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
    icon: Option<Element<'a, Message>>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::primary, message)
}

pub fn secondary_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<Element<'a, Message>>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::secondary, message)
}

pub fn danger_button<'a, Message: Clone + 'a>(
    label: &'a str,
    icon: Option<Element<'a, Message>>,
    message: Option<Message>,
) -> button::Button<'a, Message> {
    create_button(label, icon, iced::widget::button::danger, message)
}

/// Calculates progress as a value between 0.0 and 1.0.
///
/// Returns 0.0 if total is 0, otherwise returns completed/total clamped to [0.0, 1.0].
///
/// # Examples
/// ```
/// use btc_toolkit::ui_helpers::calculate_progress;
/// assert_eq!(calculate_progress(0, 0), 0.0);
/// assert_eq!(calculate_progress(50, 100), 0.5);
/// assert_eq!(calculate_progress(150, 100), 1.0); // Clamped to max
/// ```
#[inline]
pub fn calculate_progress(completed: usize, total: usize) -> f32 {
    if total == 0 {
        return 0.0;
    }

    (completed as f32 / total as f32).clamp(0.0, 1.0)
}

/// Formats a duration in seconds to a human-readable string.
///
/// # Examples
/// - 0-59 seconds: "45s"
/// - 1-59 minutes: "2m 30s"
/// - 1+ hours: "1h 30m"
pub fn format_duration(seconds: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 3600;

    match seconds {
        0..MINUTE => format!("{seconds}s"),
        MINUTE..HOUR => {
            let minutes = seconds / MINUTE;
            let secs = seconds % MINUTE;
            format!("{minutes}m {secs}s")
        }
        _ => {
            let hours = seconds / HOUR;
            let minutes = (seconds % HOUR) / MINUTE;
            format!("{hours}h {minutes}m")
        }
    }
}
