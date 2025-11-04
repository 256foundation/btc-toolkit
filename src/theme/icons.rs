/// Icon system using embedded Heroicons SVG files
/// All icons are embedded at compile-time for zero runtime cost

use iced::widget::svg;
use iced::widget::Svg;
use iced::Length;

// Embedded SVG icon data
pub const ARROW_LEFT: &[u8] = include_bytes!("../../assets/icons/arrow-left.svg");
pub const COG: &[u8] = include_bytes!("../../assets/icons/cog.svg");
pub const REFRESH: &[u8] = include_bytes!("../../assets/icons/refresh.svg");
pub const CHECK: &[u8] = include_bytes!("../../assets/icons/check.svg");
pub const WARNING: &[u8] = include_bytes!("../../assets/icons/warning.svg");
pub const ERROR: &[u8] = include_bytes!("../../assets/icons/error.svg");
pub const ADD: &[u8] = include_bytes!("../../assets/icons/add.svg");
pub const PLAY: &[u8] = include_bytes!("../../assets/icons/play.svg");
pub const STOP: &[u8] = include_bytes!("../../assets/icons/stop.svg");
pub const NETWORK: &[u8] = include_bytes!("../../assets/icons/network.svg");
pub const QUESTION_MARK: &[u8] = include_bytes!("../../assets/icons/question-mark-circle.svg");

/// Standard icon size for buttons and UI elements
pub const ICON_SIZE: u16 = 20;

/// Small icon size for compact displays
pub const ICON_SIZE_SM: u16 = 16;

/// Large icon size for headers and prominent elements
pub const ICON_SIZE_LG: u16 = 24;

/// Create an SVG icon widget with standard size
pub fn icon(data: &'static [u8]) -> Svg<'static> {
    svg(svg::Handle::from_memory(data))
        .width(Length::Fixed(ICON_SIZE as f32))
        .height(Length::Fixed(ICON_SIZE as f32))
}

/// Create an SVG icon widget with small size
pub fn icon_sm(data: &'static [u8]) -> Svg<'static> {
    svg(svg::Handle::from_memory(data))
        .width(Length::Fixed(ICON_SIZE_SM as f32))
        .height(Length::Fixed(ICON_SIZE_SM as f32))
}

/// Create an SVG icon widget with large size
pub fn icon_lg(data: &'static [u8]) -> Svg<'static> {
    svg(svg::Handle::from_memory(data))
        .width(Length::Fixed(ICON_SIZE_LG as f32))
        .height(Length::Fixed(ICON_SIZE_LG as f32))
}

/// Create an SVG icon widget with custom size
pub fn icon_size(data: &'static [u8], size: u16) -> Svg<'static> {
    svg(svg::Handle::from_memory(data))
        .width(Length::Fixed(size as f32))
        .height(Length::Fixed(size as f32))
}

// Convenience functions for common icons

pub fn back() -> Svg<'static> {
    icon(ARROW_LEFT)
}

pub fn settings() -> Svg<'static> {
    icon(COG)
}

pub fn refresh() -> Svg<'static> {
    icon(REFRESH)
}

pub fn check() -> Svg<'static> {
    icon(CHECK)
}

pub fn warning() -> Svg<'static> {
    icon(WARNING)
}

pub fn error() -> Svg<'static> {
    icon(ERROR)
}

pub fn add() -> Svg<'static> {
    icon(ADD)
}

pub fn play() -> Svg<'static> {
    icon(PLAY)
}

pub fn stop() -> Svg<'static> {
    icon(STOP)
}

pub fn network() -> Svg<'static> {
    icon(NETWORK)
}

pub fn question_mark() -> Svg<'static> {
    icon(QUESTION_MARK)
}
