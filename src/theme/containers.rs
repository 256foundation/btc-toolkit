use iced::widget::container;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn card(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette().background)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: theme.extended_palette().background.weak.color,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: Some(theme.palette().text),
    }
}

pub fn header(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            theme.extended_palette().background.base.color,
        )),
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
        text_color: Some(theme.extended_palette().background.base.text),
    }
}

pub fn success(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.2, 0.8, 0.3, 0.1))),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: theme.palette().success,
        },
        shadow: Shadow::default(),
        text_color: Some(theme.palette().text),
    }
}

pub fn error(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.9, 0.2, 0.2, 0.1))),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: theme.palette().danger,
        },
        shadow: Shadow::default(),
        text_color: Some(theme.palette().text),
    }
}

pub fn warning(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 0.8, 0.0, 0.1))),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: Color::from_rgb(1.0, 0.8, 0.0),
        },
        shadow: Shadow::default(),
        text_color: Some(theme.palette().text),
    }
}
