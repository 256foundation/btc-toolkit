use super::colors;
use iced::Font;
use iced::widget::text;

// Font sizes following a type scale
pub const TITLE_SIZE: f32 = 32.0;
pub const SUBTITLE_SIZE: f32 = 24.0;
pub const HEADING_SIZE: f32 = 20.0;
pub const SUBHEADING_SIZE: f32 = 16.0;
pub const BODY_SIZE: f32 = 14.0;
pub const SMALL_SIZE: f32 = 12.0;
pub const TINY_SIZE: f32 = 10.0;

// Font weights
pub const FONT_REGULAR: Font = Font::DEFAULT;
pub const FONT_MONO: Font = Font::MONOSPACE;

/// Large title text - for main page headers
pub fn title<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(TITLE_SIZE)
        .font(FONT_MONO)
        .color(colors::TEXT_PRIMARY)
}

/// Subtitle text - for section headers
pub fn subtitle<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(SUBTITLE_SIZE)
        .color(colors::TEXT_PRIMARY)
}

/// Heading text - for card titles and important labels
pub fn heading<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(HEADING_SIZE)
        .font(FONT_MONO)
        .color(colors::TEXT_PRIMARY)
}

/// Subheading text - for secondary headings
pub fn subheading<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(SUBHEADING_SIZE)
        .color(colors::TEXT_PRIMARY)
}

/// Body text - standard paragraph text
pub fn body<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(BODY_SIZE)
        .color(colors::TEXT_PRIMARY)
}

/// Small text - for secondary information
pub fn small<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(SMALL_SIZE)
        .color(colors::TEXT_SECONDARY)
}

/// Tiny text - for labels and minimal text
pub fn tiny<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(TINY_SIZE)
        .color(colors::TEXT_TERTIARY)
}

/// Monospace text - for IP addresses, codes, technical data
pub fn mono<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(BODY_SIZE)
        .font(FONT_MONO)
        .color(colors::TEXT_PRIMARY)
}

/// Large monospace text - for important numbers and metrics
pub fn mono_large<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(HEADING_SIZE)
        .font(FONT_MONO)
        .color(colors::TEXT_PRIMARY)
}

/// Extra large monospace - for big metrics display
pub fn mono_xl<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(SUBTITLE_SIZE)
        .font(FONT_MONO)
        .color(colors::TEXT_PRIMARY)
}

// Colored text helpers

// Theme API functions - may be unused but part of design system
#[allow(dead_code)]
/// Success text - green for positive indicators
pub fn success<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).color(colors::SUCCESS)
}

#[allow(dead_code)]
/// Warning text - orange for caution
pub fn warning<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).color(colors::WARNING)
}

#[allow(dead_code)]
/// Danger text - red for errors
pub fn danger<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).color(colors::DANGER)
}

#[allow(dead_code)]
/// Primary colored text - industrial blue
pub fn primary<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).color(colors::PRIMARY)
}

#[allow(dead_code)]
/// Accent colored text - bright cyan
pub fn accent<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).color(colors::ACCENT)
}

#[allow(dead_code)]
/// Disabled text
pub fn disabled<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(BODY_SIZE)
        .color(colors::TEXT_DISABLED)
}

#[allow(dead_code)]
/// Label text with icon - combines icon and text
pub fn with_icon<T: Into<String>>(icon: &str, content: T) -> String {
    format!("{} {}", icon, content.into())
}
