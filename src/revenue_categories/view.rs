use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};
use crate::data_types::{EntityId, ValidationError};
use crate::HotKey;
use super::RevenueCategory;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view(category: &RevenueCategory) -> Element<Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&category.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(category.id.to_string())
            ],
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    container(
        column![
            header,
            content,
        ]
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