use iced::widget::{
    button, column, container, row, text, text_input, pick_list,
    horizontal_space,
};
use iced::{Element, Length};
use rust_decimal::Decimal;
use std::collections::HashMap;
use crate::data_types::{EntityId, ValidationError, Currency};
use crate::HotKey;
use super::{PriceLevel, PriceLevelType};

#[derive(Debug, Clone)]
pub enum Message {
   UpdateName(String),
   UpdateId(String),
   UpdatePrice(String),
   UpdateType(PriceLevelType),
   Save,
   Cancel,
}

pub fn view<'a>(
    price_level: &'a PriceLevel,
    state: super::EditState,
    all_levels: &'a HashMap<EntityId, PriceLevel>
) -> Element<'a, Message> {

    let validation_error = &state.validation_error;

    let other_levels: Vec<&PriceLevel> = all_levels.values()
    .filter(|l| l.id != price_level.id)
    .collect();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Price Level Name", &price_level.name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID (1-999)", &price_level.id.to_string())
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Price").width(Length::Fixed(150.0)),
                text_input("Price", &price_level.price.to_string())
                    .on_input(Message::UpdatePrice)
                    .padding(5)
            ],
            row![
                text("Type").width(Length::Fixed(150.0)),
                pick_list(
                    vec![
                        PriceLevelType::Enterprise,
                        PriceLevelType::Store,
                    ],
                    Some(price_level.level_type.clone()),
                    Message::UpdateType
                )
            ],
            // Show validation error if any
            if let Some(error) = validation_error {
                text(error.to_string()).style(text::danger)
            } else {
                text("".to_string())
            },
            row![
                horizontal_space(),
                button("Cancel")
                    .on_press(Message::Cancel)
                    .style(button::danger),
                button("Save")
                    .on_press(Message::Save)
                    .style(button::success)
            ].spacing(10)
        ]
        .spacing(10)
    )
    .padding(20);
 
    container(content).into()
 }