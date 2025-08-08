use iced::Font;
#[allow(dead_code)]
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
