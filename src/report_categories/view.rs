use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};
use crate::icon;
use crate::HotKey;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view<'a>(report_category: &'a super::ReportCategory) -> Element<'a, Message> {
    let header = row![
        horizontal_space().width(40),
        text(&report_category.name).size(16),
        horizontal_space(),
        button(icon::edit().shaping(text::Shaping::Advanced)).on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(report_category.id.to_string())
            ],
            row![
                text("Name:").width(Length::Fixed(150.0)), 
                text(&report_category.name)
            ]
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    container(
        column![header, content]
            .spacing(20)
    )
    .padding(20)
    .into()
}

pub fn handle_hotkey(hotkey: HotKey) -> crate::Action<super::Operation, Message> {
    match hotkey {
        HotKey::Escape => crate::Action::operation(super::Operation::Back),
        _ => crate::Action::none(),
    }
}