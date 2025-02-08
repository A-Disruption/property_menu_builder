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
use std::collections::HashMap;

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

#[derive(Default, Clone)]
pub struct EditState {
    pub name: String,
    pub id: String,
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(report_category: &ReportCategory) -> Self {
        Self {
            name: report_category.name.clone(),
            id: report_category.id.to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Report category name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Report category ID must be between 1 and 999".to_string()
                ));
            }
        } else {
            return Err(ValidationError::InvalidId(
                "Invalid ID format".to_string()
            ));
        }

        Ok(())
    }
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

impl Default for ReportCategory {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
        }
    }
}

impl ReportCategory {
    fn validate(&self, other_categories: &[&ReportCategory]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Report category ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Report category with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Report category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}


pub fn update(
    report_category: &mut ReportCategory,
    message: Message,
    state: &mut EditState,
    other_categories: &[&ReportCategory]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                report_category.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    report_category.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if report_category.validate(other_categories).is_ok() {
                    Action::operation(Operation::Save(report_category.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(report_category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}


pub fn view<'a>(
    report_category: &'a ReportCategory, 
    mode: &'a Mode,
    all_categories: &'a HashMap<EntityId, ReportCategory>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(report_category).map(Message::View),
        Mode::Edit => {
            edit::view(
                report_category,
                EditState::new(report_category),
                all_categories
            ).map(Message::Edit)
        }
    }
}