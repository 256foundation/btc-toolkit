use iced::widget::container;
use iced::{Background, Border, Color, Shadow, Theme, Vector};
use super::colors;

/// Card style - elevated surface for content sections
pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BACKGROUND_CARD)),
        border: Border {
            radius: 8.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: colors::SHADOW_LIGHT,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Header style - top navigation and section headers
pub fn header(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BACKGROUND_ELEVATED)),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: colors::SHADOW_MEDIUM,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Success style - positive status indicators
pub fn success(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SUCCESS_DIM)),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: colors::SUCCESS,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.8, 0.4, 0.2),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Error style - error states and critical alerts
pub fn error(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::DANGER_DIM)),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: colors::DANGER,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.95, 0.26, 0.21, 0.2),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Warning style - caution and important notices
pub fn warning(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::WARNING_DIM)),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: colors::WARNING,
        },
        shadow: Shadow {
            color: Color::from_rgba(1.0, 0.65, 0.0, 0.2),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Primary style - emphasized content
pub fn primary(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::PRIMARY)),
        border: Border {
            radius: 6.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: colors::SHADOW_MEDIUM,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
        text_color: Some(colors::TEXT_ON_PRIMARY),
    }
}

/// Accent style - highlights and call-to-actions
pub fn accent(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::ACCENT)),
        border: Border {
            radius: 6.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: colors::SHADOW_MEDIUM,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
        text_color: Some(colors::TEXT_ON_PRIMARY),
    }
}

/// Transparent style - borderless containers
pub fn transparent(_theme: &Theme) -> container::Style {
    container::Style {
        background: None,
        border: Border::default(),
        shadow: Shadow::default(),
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Status badge style - compact status indicators
pub fn badge(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_OVERLAY_20)),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: colors::BORDER_DEFAULT,
        },
        shadow: Shadow::default(),
        text_color: Some(colors::TEXT_PRIMARY),
    }
}

/// Tooltip style - hovering information boxes
pub fn tooltip(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::BACKGROUND_ELEVATED)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: colors::BORDER_STRONG,
        },
        shadow: Shadow {
            color: colors::SHADOW_HEAVY,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        text_color: Some(colors::TEXT_PRIMARY),
    }
}
