use iced::widget::{button, checkbox, container, progress_bar, text_input};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

/// Bitcoin-inspired color palette optimized for industrial/technical use
#[derive(Debug, Clone)]
pub struct BtcTheme {
    /// Primary Bitcoin orange
    pub primary: Color,
    /// Dark Bitcoin orange for hover states
    pub primary_dark: Color,
    /// Light Bitcoin orange for backgrounds
    pub primary_light: Color,

    /// Professional dark backgrounds
    pub background: Color,
    pub surface: Color,
    pub card: Color,

    /// Text colors for excellent readability
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,

    /// Status colors for mining operations
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

    /// Mining-specific colors
    pub miner_online: Color,
    pub miner_offline: Color,
    pub scanning: Color,

    /// Border and divider colors
    pub border: Color,
    pub border_light: Color,
}

impl Default for BtcTheme {
    fn default() -> Self {
        Self {
            // Bitcoin orange palette
            primary: Color::from_rgb(0.96, 0.59, 0.09), // #F59816 - Bitcoin orange
            primary_dark: Color::from_rgb(0.85, 0.51, 0.07), // #DA8112 - Darker orange
            primary_light: Color::from_rgb(1.0, 0.93, 0.8), // #FFEDCC - Light orange background

            // Professional dark theme for reduced eye strain
            background: Color::from_rgb(0.08, 0.08, 0.08), // #141414 - Very dark background
            surface: Color::from_rgb(0.12, 0.12, 0.12),    // #1F1F1F - Surface color
            card: Color::from_rgb(0.16, 0.16, 0.16),       // #292929 - Card background

            // High contrast text for readability in industrial environments
            text_primary: Color::from_rgb(0.95, 0.95, 0.95), // #F2F2F2 - Primary text
            text_secondary: Color::from_rgb(0.8, 0.8, 0.8),  // #CCCCCC - Secondary text
            text_muted: Color::from_rgb(0.6, 0.6, 0.6),      // #999999 - Muted text

            // Status colors optimized for mining operations
            success: Color::from_rgb(0.2, 0.8, 0.3), // #33CC52 - Success green
            warning: Color::from_rgb(1.0, 0.8, 0.0), // #FFCC00 - Warning yellow
            error: Color::from_rgb(0.9, 0.2, 0.2),   // #E63333 - Error red
            info: Color::from_rgb(0.2, 0.6, 1.0),    // #3399FF - Info blue

            // Mining status colors
            miner_online: Color::from_rgb(0.2, 0.8, 0.3), // #33CC52 - Online green
            miner_offline: Color::from_rgb(0.6, 0.6, 0.6), // #999999 - Offline gray
            scanning: Color::from_rgb(0.96, 0.59, 0.09),  // #F59816 - Scanning orange

            // Borders for clean separation
            border: Color::from_rgb(0.3, 0.3, 0.3), // #4D4D4D - Primary border
            border_light: Color::from_rgb(0.2, 0.2, 0.2), // #333333 - Light border
        }
    }
}

/// Create a custom Iced theme based on our Bitcoin color scheme
pub fn btc_theme() -> Theme {
    Theme::Dark
}

/// Button styles for different use cases
pub mod button_styles {
    use super::*;

    pub fn primary(_theme: &Theme, _status: button::Status) -> button::Style {
        let btc_theme = BtcTheme::default();
        button::Style {
            background: Some(Background::Color(btc_theme.primary)),
            border: Border {
                radius: 6.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::BLACK,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        }
    }

    pub fn secondary(_theme: &Theme, _status: button::Status) -> button::Style {
        let btc_theme = BtcTheme::default();
        button::Style {
            background: Some(Background::Color(btc_theme.surface)),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: btc_theme.border,
            },
            text_color: btc_theme.text_primary,
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        }
    }

    pub fn danger(_theme: &Theme, _status: button::Status) -> button::Style {
        let btc_theme = BtcTheme::default();
        button::Style {
            background: Some(Background::Color(btc_theme.error)),
            border: Border {
                radius: 6.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::WHITE,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        }
    }

    pub fn text_link(_theme: &Theme, _status: button::Status) -> button::Style {
        let btc_theme = BtcTheme::default();
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: btc_theme.primary,
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        }
    }
}

/// Container styles for different UI components
pub mod container_styles {
    use super::*;

    pub fn card(_theme: &Theme) -> container::Style {
        let btc_theme = BtcTheme::default();
        container::Style {
            background: Some(Background::Color(btc_theme.card)),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: btc_theme.border_light,
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            text_color: Some(btc_theme.text_primary),
        }
    }

    pub fn header(_theme: &Theme) -> container::Style {
        let btc_theme = BtcTheme::default();
        container::Style {
            background: Some(Background::Color(btc_theme.surface)),
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            text_color: Some(btc_theme.text_primary),
        }
    }

    pub fn status_success(_theme: &Theme) -> container::Style {
        let btc_theme = BtcTheme::default();
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.8, 0.3, 0.1))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: btc_theme.success,
            },
            shadow: Shadow::default(),
            text_color: Some(btc_theme.success),
        }
    }

    pub fn status_error(_theme: &Theme) -> container::Style {
        let btc_theme = BtcTheme::default();
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0.9, 0.2, 0.2, 0.1))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: btc_theme.error,
            },
            shadow: Shadow::default(),
            text_color: Some(btc_theme.error),
        }
    }

    pub fn status_warning(_theme: &Theme) -> container::Style {
        let btc_theme = BtcTheme::default();
        container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 0.8, 0.0, 0.1))),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: btc_theme.warning,
            },
            shadow: Shadow::default(),
            text_color: Some(btc_theme.warning),
        }
    }
}

/// Text input styles
pub mod text_input_styles {
    use super::*;

    pub fn default(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
        let btc_theme = BtcTheme::default();
        text_input::Style {
            background: Background::Color(btc_theme.surface),
            border: Border {
                radius: 6.0.into(),
                width: 1.0,
                color: btc_theme.border,
            },
            icon: btc_theme.text_muted,
            placeholder: btc_theme.text_muted,
            value: btc_theme.text_primary,
            selection: btc_theme.primary_light,
        }
    }
}

/// Progress bar styles for scan progress
pub mod progress_bar_styles {
    use super::*;

    pub fn scanning(_theme: &Theme) -> progress_bar::Style {
        let btc_theme = BtcTheme::default();
        progress_bar::Style {
            background: Background::Color(btc_theme.surface),
            bar: Background::Color(btc_theme.primary),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        }
    }
}

/// Checkbox styles
pub mod checkbox_styles {
    use super::*;

    pub fn default(_theme: &Theme, _status: checkbox::Status) -> checkbox::Style {
        let btc_theme = BtcTheme::default();
        checkbox::Style {
            background: Background::Color(btc_theme.surface),
            icon_color: btc_theme.primary,
            border: Border {
                radius: 4.0.into(),
                width: 1.0,
                color: btc_theme.border,
            },
            text_color: Some(btc_theme.text_primary),
        }
    }
}

/// Typography utilities for consistent text sizing and weights
pub mod typography {
    use iced::Font;
    use iced::widget::text;

    /// Font sizes optimized for technician readability
    pub const TITLE_SIZE: u16 = 28;
    pub const SUBTITLE_SIZE: u16 = 20;
    pub const HEADING_SIZE: u16 = 18;
    pub const BODY_SIZE: u16 = 14;
    pub const SMALL_SIZE: u16 = 12;
    pub const TINY_SIZE: u16 = 10;

    /// Create title text with consistent styling
    pub fn title<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(TITLE_SIZE).font(Font::MONOSPACE) // Monospace for technical precision
    }

    /// Create subtitle text
    pub fn subtitle<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(SUBTITLE_SIZE)
    }

    /// Create heading text
    pub fn heading<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into())
            .size(HEADING_SIZE)
            .font(Font::MONOSPACE)
    }

    /// Create body text
    pub fn body<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(BODY_SIZE)
    }

    /// Create small text for additional info
    pub fn small<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(SMALL_SIZE)
    }

    /// Create tiny text for minor details
    pub fn tiny<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(TINY_SIZE)
    }

    /// Create monospace text for IP addresses, technical data
    pub fn mono<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into()).size(BODY_SIZE).font(Font::MONOSPACE)
    }

    /// Create large monospace text for important technical data
    pub fn mono_large<T: Into<String>>(content: T) -> text::Text<'static> {
        text(content.into())
            .size(HEADING_SIZE)
            .font(Font::MONOSPACE)
    }
}

/// Layout utilities for consistent spacing and sizing
pub mod layout {
    use iced::Length;

    /// Standard padding values
    pub const PADDING_XS: f32 = 4.0;
    pub const PADDING_SM: f32 = 8.0;
    pub const PADDING_MD: f32 = 16.0;
    pub const PADDING_LG: f32 = 24.0;
    pub const PADDING_XL: f32 = 32.0;

    /// Standard spacing values
    pub const SPACING_XS: f32 = 4.0;
    pub const SPACING_SM: f32 = 8.0;
    pub const SPACING_MD: f32 = 16.0;
    pub const SPACING_LG: f32 = 24.0;
    pub const SPACING_XL: f32 = 32.0;

    /// Common width values for consistent sizing
    pub const BUTTON_WIDTH: Length = Length::Fixed(120.0);
    pub const LABEL_WIDTH: Length = Length::Fixed(140.0);
    pub const INPUT_WIDTH: Length = Length::Fixed(200.0);
    pub const ICON_SIZE: Length = Length::Fixed(20.0);
}

/// Status indicators for mining operations
pub mod status {
    use super::*;

    /// Get color for miner status
    pub fn miner_status_color(is_online: bool) -> Color {
        let theme = BtcTheme::default();
        if is_online {
            theme.miner_online
        } else {
            theme.miner_offline
        }
    }

    /// Get status icon for mining operations
    pub fn status_icon(status: &str) -> &'static str {
        match status {
            "online" | "active" | "complete" => "✅",
            "offline" | "inactive" | "error" => "❌",
            "scanning" | "pending" | "loading" => "🔄",
            "warning" | "throttled" => "⚠️",
            _ => "⚪",
        }
    }
}
