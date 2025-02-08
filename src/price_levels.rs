pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Currency,
    Validatable,
    ValidationError
};
use crate::Action;
use iced::Element;
use std::collections::HashMap;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(PriceLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(PriceLevel),
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
}

#[derive(Default, Clone)]
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

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Price level ID must be between 1 and 999".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        if let Err(_) = self.price.parse::<Decimal>() {
            return Err(ValidationError::InvalidValue(
                "Invalid price format".to_string()
            ));
        }

        Ok(())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PriceLevelType {
    Enterprise,
    Store
}

impl Default for PriceLevelType {
    fn default() -> Self {
        Self::Enterprise
    }
}

impl std::fmt::Display for PriceLevelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PriceLevelType::Enterprise => write!(f, "Enterprise Price Level"),
            PriceLevelType::Store => write!(f, "Store Price Level"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl Default for PriceLevel {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
            price: Decimal::ZERO,
            level_type: PriceLevelType::default(),
        }
    }
}

impl PriceLevel {

    pub fn new_draft() -> Self {
        Self {
            id: -1,  // Temporary UI-only ID
            name: String::new(),
            ..PriceLevel::default()
        } 
    }

    fn validate(&self, other_levels: &[&PriceLevel]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Price level ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Price level with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price level name cannot be empty".to_string()
            ));
        }

        if self.price < Decimal::ZERO {
            return Err(ValidationError::InvalidValue(
                "Price cannot be negative".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    price_level: &mut PriceLevel,
    message: Message,
    state: &mut EditState,
    other_levels: &[&PriceLevel]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                price_level.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    price_level.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::UpdatePrice(price_str) => {
                match price_str.parse() {
                    Ok(price) => {
                        price_level.price = price;
                        Action::none()
                    }
                    Err(_) => {
                        state.validation_error = Some("Invalid price format".to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::UpdateType(level_type) => {
                price_level.level_type = level_type;
                Action::none()
            }
            edit::Message::Save => {
                if price_level.validate(other_levels).is_ok() {
                    Action::operation(Operation::Save(price_level.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(price_level.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_price_level = PriceLevel::default();
            Action::operation(Operation::CreateNew(new_price_level))
        },
        Message::Select => {
            Action::operation(Operation::Select(price_level.id))
        },
    }
}

pub fn view<'a>(
    price_level: &'a PriceLevel, 
    mode: &'a Mode,
    all_levels: &'a HashMap<EntityId, PriceLevel>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(price_level).map(Message::View),
        Mode::Edit => {
            edit::view(
                price_level,
                EditState::new(price_level),
                all_levels
            ).map(Message::Edit)
        }
    }
}