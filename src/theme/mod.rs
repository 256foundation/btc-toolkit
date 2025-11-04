use iced::Theme;

/// Custom industrial theme for BTC mining operations
/// Dark theme optimized for long viewing sessions with high-contrast elements
pub fn industrial_theme() -> Theme {
    Theme::custom(
        "Industrial".to_string(),
        iced::theme::Palette {
            background: colors::BACKGROUND_BASE,
            text: colors::TEXT_PRIMARY,
            primary: colors::PRIMARY,
            success: colors::SUCCESS,
            danger: colors::DANGER,
        }
    )
}

/// The application theme - Industrial dark theme
pub(crate) fn theme() -> Theme {
    industrial_theme()
}

pub mod colors;
pub mod icons;
pub mod containers;
pub mod padding;
pub mod spacing;
pub mod typography;
