pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use iced::Element;
use std::collections::HashMap;
use rust_decimal::Decimal;
use std::fmt;


#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(TaxGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
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
    pub rate: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(tax_group: &TaxGroup) -> Self {
        Self {
            name: tax_group.name.clone(),
            id: tax_group.id.to_string(),
            rate: tax_group.rate_percentage().to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Tax group name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=99).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Tax group ID must be between 1 and 99".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        match self.rate.parse::<Decimal>() {
            Ok(rate) => {
                if !(Decimal::ZERO..=Decimal::from(100)).contains(&rate) {
                    return Err(ValidationError::InvalidValue(
                        "Tax rate must be between 0 and 100%".to_string()
                    ));
                }
            }
            Err(_) => {
                return Err(ValidationError::InvalidValue(
                    "Invalid tax rate format".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaxGroup {
    pub id: EntityId,
    pub name: String,
    pub rate: Decimal, // Stored as decimal (e.g., 0.08 for 8%)
}

impl fmt::Display for TaxGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for TaxGroup {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
            rate: Decimal::ZERO,
        }
    }
}

impl TaxGroup {
    fn validate(&self, other_groups: &[&TaxGroup]) -> Result<(), ValidationError> {
        if !(1..=99).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Tax group ID must be between 1 and 99".to_string()
            ));
        }

        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Tax group with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Tax group name cannot be empty".to_string()
            ));
        }

        if !(Decimal::ZERO..=Decimal::ONE).contains(&self.rate) {
            return Err(ValidationError::InvalidValue(
                "Tax rate must be between 0 and 100%".to_string()
            ));
        }

        Ok(())
    }

    // Helper method to get rate as percentage
    pub fn rate_percentage(&self) -> Decimal {
        self.rate * Decimal::from(100)
    }
}

pub fn update(
    tax_group: &mut TaxGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&TaxGroup]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                tax_group.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    tax_group.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::UpdateRate(rate_str) => {
                match rate_str.parse::<Decimal>() {
                    Ok(rate_percentage) => {
                        tax_group.rate = rate_percentage / Decimal::from(100);
                        Action::none()
                    }
                    Err(_) => {
                        state.validation_error = Some("Invalid tax rate format".to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Save => {
                if tax_group.validate(other_groups).is_ok() {
                    Action::operation(Operation::Save(tax_group.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(tax_group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view<'a>(
    tax_group: &'a TaxGroup, 
    mode: &'a Mode,
    all_groups: &'a HashMap<EntityId, TaxGroup>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(tax_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                tax_group,
                EditState::new(tax_group),
                all_groups
            ).map(Message::Edit)
        }
    }
}