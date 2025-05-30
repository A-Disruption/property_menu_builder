// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/menu-builder.toml
// 97d7f9fc119a0404561e1cbd375c47e859b08feaad125794df990d040015c9fd
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/menu-builder.ttf");

pub fn cancel<'a>() -> Text<'a> {
    icon("\u{2715}")
}

pub fn copy<'a>() -> Text<'a> {
    icon("\u{F24D}")
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

pub fn superpowers<'a>() -> Text<'a> {
    icon("\u{F2DD}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E006}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("menu-builder"))
}
