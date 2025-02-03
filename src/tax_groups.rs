pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
    IdRange,
};
use crate::Action;
use iced::Element;
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

impl TaxGroup {
    fn validate(&self, other_groups: &[&TaxGroup]) -> Result<(), ValidationError> {
        // Validate ID range (1-99 as per your screenshot)
        if !(1..=99).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Tax Group ID must be between 1 and 99".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Tax Group with ID {} already exists", self.id)
                ));
            }
        }

        // Validate tax rate (0-100%)
        if self.rate < Decimal::ZERO || self.rate > Decimal::ONE {
            return Err(ValidationError::InvalidRate(
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
    state: &mut edit::EditState,
    other_groups: &[&TaxGroup],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                state.name = name;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                state.id = id;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::UpdateRate(rate) => {
                state.rate = rate;
                state.validation_error = None;
                Action::none()
            }
            edit::Message::Save => {
                match state.validate(other_groups) {
                    Ok(_) => Action::operation(Operation::Save(tax_group.clone())),
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(tax_group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        },
    }
}


pub fn view<'a>(
    taxgroup: &'a TaxGroup, 
    mode: &'a Mode,
    other_groups: &'a [&'a TaxGroup]
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(taxgroup).map(Message::View),
        Mode::Edit => {
            edit::view(
                taxgroup,
                edit::EditState::new(taxgroup),
                other_groups
            ).map(Message::Edit)
        }
    }
}