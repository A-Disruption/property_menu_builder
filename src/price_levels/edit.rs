use iced::widget::{
    button, column, container, row, text, text_input, pick_list,
    horizontal_space,
};
use iced::{Element, Length, Color};
use std::iter::empty;
use rust_decimal::Decimal;
use crate::data_types::{EntityId};
use crate::HotKey;
use super::{PriceLevel, PriceLevelType, ValidationError};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateName(String),
    UpdateId(String),
    UpdatePrice(String),
    UpdateType(PriceLevelType),
    Save,
    Cancel,
}

pub struct EditState {
    name: String,
    id: String,
    price: String,
    level_type: PriceLevelType,
    validation_error: Option<String>,
}

impl EditState {
    pub fn new(price_level: &PriceLevel) -> Self {
        Self {
            name: price_level.name.clone(),
            id: price_level.id.to_string(),
            price: price_level.price.to_string(),
            level_type: price_level.level_type.clone(),
            validation_error: None,
        }
    }
}

pub fn view(state: &EditState) -> Element<Message> {
    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Price Level Name", &state.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID", &state.id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Price").width(Length::Fixed(150.0)),
                text_input("Price", &state.price)
                    .on_input(Message::UpdatePrice)
                    .padding(5)
            ],
            row![
                text("Type").width(Length::Fixed(150.0)),
                pick_list(
                    &[PriceLevelType::Item, PriceLevelType::Store][..],
                    Some(state.level_type.clone()),
                    Message::UpdateType
                )
            ],
            if let Some(error) = &state.validation_error {
                container(
                    text(error)
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            } else {
                container(
                    text("")
                        .style(iced::widget::text::danger)
                )
                .padding(10)
            }
        ]
        .spacing(10)
    )
    .style(container::rounded_box)
    .padding(20);

    let controls = row![
        horizontal_space(),
        button("Cancel")
            .on_press(Message::Cancel)
            .style(button::danger),
        button("Save")
            .on_press(Message::Save)
            .style(button::success),
    ]
    .spacing(10)
    .padding(20);

    container(
        column![
            content,
            controls,
        ]
        .spacing(20)
    )
    .padding(20)
    .into()
}