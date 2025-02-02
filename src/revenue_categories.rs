pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    IdRange,
};
use crate::Action;
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(RevenueCategory),
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
    EmptyName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RevenueCategory {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for RevenueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl RevenueCategory {
    fn validate(&self, other_categories: &[&RevenueCategory]) -> Result<(), ValidationError> {
        // Validate ID range (1-99 based on your screenshot)
        if !(1..=99).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Revenue Category ID must be between 1 and 99".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Revenue Category with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Revenue Category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    category: &mut RevenueCategory,
    message: Message,
    state: &mut edit::EditState,
    other_categories: &[&RevenueCategory],
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
            edit::Message::Save => {
                match state.validate(other_categories) {
                    Ok(_) => Action::operation(Operation::Save(category.clone())),
                    Err(e) => {
                        state.validation_error = Some(e.to_string());
                        Action::none()
                    }
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        },
    }
}

pub fn view<'a>(
    category: &'a RevenueCategory, 
    mode: Mode,
    state: &'a edit::EditState,
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(category).map(Message::View),
        Mode::Edit => edit::view(state).map(Message::Edit),
    }
}