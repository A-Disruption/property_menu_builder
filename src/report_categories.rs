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

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ReportCategory),
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
pub struct ReportCategory {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ReportCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ReportCategory {
    fn validate(&self, other_categories: &[&ReportCategory]) -> Result<(), ValidationError> {
        // Validate ID range (1-255 based on your screenshot)
        if !(1..=255).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Report Category ID must be between 1 and 255".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Report Category with ID {} already exists", self.id)
                ));
            }
        }

        // Validate name is not empty
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Report Category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    category: &mut ReportCategory,
    message: Message,
    other_categories: &[&ReportCategory],
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::Save => {
                match category.validate(other_categories) {
                    Ok(_) => Action::operation(Operation::Save(category.clone())),
                    Err(e) => Action::none(), // Error will be shown in UI
                }
            },
            edit::Message::Cancel => Action::operation(Operation::Cancel),
            // Other edit messages handled by edit::update
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view(category: &ReportCategory, mode: Mode) -> Element<Message> {
    match mode {
        Mode::View => view::view(category).map(Message::View),
        Mode::Edit => edit::view(category).map(Message::Edit),
    }
}