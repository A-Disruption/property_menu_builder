use iced::widget::{
    button, column, container, row, text,
    horizontal_space,
};
use iced::{Alignment, Element, Length};
use crate::icon;

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    Back,
}

pub fn view<'a>(price_level: &'a super::PriceLevel) -> Element<'a, Message> {
    let header = row![
        horizontal_space().width(40),
        text(&price_level.name).size(16),
        horizontal_space(),
        button(icon::edit().size(14)).on_press(Message::Edit)
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
                text("Name:").width(Length::Fixed(150.0)), 
                text(&price_level.name)
            ],
            row![
                text("Price:").width(Length::Fixed(150.0)),
                text(format!("${:.2}", price_level.price))
            ],
            row![
                text("Type:").width(Length::Fixed(150.0)),
                text(price_level.level_type.to_string())
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