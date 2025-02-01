use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};

use crate::HotKey;
use super::PriceLevel;
use crate::data_types::{EntityId, ValidationError};
#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view(price_level: &PriceLevel) -> Element<Message> {
    let header = row![
        button("←").width(40).on_press(Message::Back),
        text(&price_level.name).size(16),
        horizontal_space(),
        button("Edit").on_press(Message::Edit)
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let content = container(
        column![
            row![
                text("ID:").width(Length::Fixed(150.0)),
                text(price_level.id.to_string())
            ],
            row![
                text("Price:").width(Length::Fixed(150.0)),
                text(format!("${:.2}", price_level.price))
            ],
            row![
                text("Type:").width(Length::Fixed(150.0)),
                text(format!("{:?}", price_level.level_type))
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