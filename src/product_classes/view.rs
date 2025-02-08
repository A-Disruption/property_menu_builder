use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};

use crate::HotKey;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view<'a>(product_class: &'a super::ProductClass) -> Element<'a, Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&product_class.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(product_class.id.to_string())
            ],
            row![
                text("Name:").width(Length::Fixed(150.0)), 
                text(&product_class.name)
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