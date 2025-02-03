use iced::widget::{
    button, column, container, row, text, text_input, pick_list,
    horizontal_space,
};
use iced::{Element, Length};
use rust_decimal::Decimal;
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

pub struct EditState {
    pub name: String,
    pub id: String,
    pub price: String,
    pub level_type: PriceLevelType,
    pub validation_error: Option<String>,
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

impl EditState {
    pub fn validate(&self, other_levels: &[&PriceLevel]) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        let id: EntityId = self.id.parse().map_err(|_| {
            ValidationError::InvalidId("Invalid ID format".to_string())
        })?;

        // Validate ID range based on level type
        match self.level_type {
            PriceLevelType::Item if !(1..=999).contains(&id) => {
                return Err(ValidationError::InvalidId(
                    "Item Price Level ID must be between 1 and 999".to_string()
                ))
            }
            PriceLevelType::Store if !(1..=99999).contains(&id) => {
                return Err(ValidationError::InvalidId(
                    "Store Price Level ID must be between 1 and 99999".to_string()
                ))
            }
            _ => {}
        }

        let price: Currency = self.price.parse().map_err(|_| {
            ValidationError::InvalidValue("Invalid price format".to_string())
        })?;

        if price < Currency::ZERO {
            return Err(ValidationError::InvalidValue(
                "Price cannot be negative".to_string()
            ));
        }

        for other in other_levels {
            if id == other.id && self.level_type == other.level_type {
                return Err(ValidationError::DuplicateId(
                    format!("Price Level with ID {} already exists for this type", id)
                ));
            }
        }

        Ok(())
    }
}

pub fn view<'a>(
    level: &'a PriceLevel,
    state: EditState,
    other_levels: &'a [&'a PriceLevel],
) -> Element<'a, Message> {
    
    let name = state.name.clone();
    let id = state.id.clone();
    let price = state.price.clone();
    let level_type = state.level_type.clone();
    let error_message = state.validation_error.clone();

    let content = container(
        column![
            row![
                text("Name").width(Length::Fixed(150.0)),
                text_input("Price Level Name", &name)
                    .on_input(Message::UpdateName)
                    .padding(5)
            ],
            row![
                text("ID").width(Length::Fixed(150.0)),
                text_input("ID", &id)
                    .on_input(Message::UpdateId)
                    .padding(5)
            ],
            row![
                text("Price").width(Length::Fixed(150.0)),
                text_input("Price", &price)
                    .on_input(Message::UpdatePrice)
                    .padding(5)
            ],
            row![
                text("Type").width(Length::Fixed(150.0)),
                pick_list(
                    &[PriceLevelType::Item, PriceLevelType::Store][..],
                    Some(level_type),
                    Message::UpdateType
                )
            ],
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

    let mut col = column![content, controls].spacing(20);

    if let Some(error) = error_message {
        col = col.push(
            container(
                text(error)
                    .style(text::danger)
            )
            .padding(10)
        );
    }

    container(col)
        .padding(20)
        .into()
}