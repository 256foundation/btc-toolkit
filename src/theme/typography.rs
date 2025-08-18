use iced::Font;
use iced::widget::text;

pub const TITLE_SIZE: u16 = 28;
pub const SUBTITLE_SIZE: u16 = 20;
pub const HEADING_SIZE: u16 = 18;
pub const BODY_SIZE: u16 = 14;
pub const SMALL_SIZE: u16 = 12;
pub const TINY_SIZE: u16 = 10;

pub fn title<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(TITLE_SIZE).font(Font::MONOSPACE)
}

pub fn subtitle<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(SUBTITLE_SIZE)
}

pub fn heading<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(HEADING_SIZE)
        .font(Font::MONOSPACE)
}

pub fn body<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE)
}

pub fn small<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(SMALL_SIZE)
}

pub fn tiny<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(TINY_SIZE)
}

pub fn mono<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into()).size(BODY_SIZE).font(Font::MONOSPACE)
}

pub fn mono_large<T: Into<String>>(content: T) -> text::Text<'static> {
    text(content.into())
        .size(HEADING_SIZE)
        .font(Font::MONOSPACE)
}
