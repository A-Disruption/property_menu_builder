// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/menu-builder.toml
// 534e1dada7e281c7bec95c6fba7fbc12bb27db597420517eec37b7c5e2468845
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/menu-builder.ttf");

pub fn cancel<'a>() -> Text<'a> {
    icon("\u{2715}")
}

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn home<'a>() -> Text<'a> {
    icon("\u{2302}")
}

pub fn new<'a>() -> Text<'a> {
    icon("\u{2B}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn search<'a>() -> Text<'a> {
    icon("\u{1F50D}")
}

pub fn settings<'a>() -> Text<'a> {
    icon("\u{2699}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{F1F8}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("menu-builder"))
}
