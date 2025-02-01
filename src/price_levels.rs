pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Currency,
    Validatable,
    IdRange,
};
use crate::Action;
use iced::Element;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PriceLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    InvalidPrice(String),
}

#[derive(Debug, Clone)]
pub struct PriceLevel {
    pub id: EntityId,
    pub name: String,
    pub price: Currency,
    pub level_type: PriceLevelType,
}

impl std::fmt::Display for PriceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriceLevelType {
    Item,     // Valid range: 1-999
    Store,    // Valid range: 1-99999
}

impl std::fmt::Display for PriceLevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriceLevelType::Item => write!(f, "Item Price Level"),
            PriceLevelType::Store => write!(f, "Store Price Level"),
        }
    }
}

impl PriceLevel {
    fn validate(&self, other_levels: &[&PriceLevel]) -> Result<(), ValidationError> {
        // Validate ID range based on type
        let valid_range = match self.level_type {
            PriceLevelType::Item => 1..=999,
            PriceLevelType::Store => 1..=99999,
        };

        if !valid_range.contains(&self.id) {
            return Err(ValidationError::InvalidId(
                format!("ID must be between {} and {} for {:?} price levels",
                    valid_range.start(), valid_range.end(), self.level_type)
            ));
        }

        // Check for duplicate IDs within the same type
        for other in other_levels {
            if other.id == self.id && other.level_type == self.level_type {
                return Err(ValidationError::DuplicateId(
                    format!("Price level with ID {} already exists", self.id)
                ));
            }
        }

        // Validate price is non-negative
        if self.price < Decimal::ZERO {
            return Err(ValidationError::InvalidPrice(
                "Price cannot be negative".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(price_level: &mut PriceLevel, message: Message, other_levels: &[&PriceLevel]) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::Save => {
                match price_level.validate(other_levels) {
                    Ok(_) => Action::operation(Operation::Save(price_level.clone())),
                    Err(e) => Action::none(), // Handle error in UI
                }
            },
            edit::Message::Cancel => Action::operation(Operation::Cancel),
            // Other edit messages will be handled by edit::update
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(price_level.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view(price_level: &PriceLevel, mode: Mode) -> Element<Message> {
    match mode {
        Mode::View => view::view(price_level).map(Message::View),
        Mode::Edit => edit::view(price_level).map(Message::Edit),
    }
}