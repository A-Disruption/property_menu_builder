pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    ValidationError,
};
use crate::Action;
use iced::Element;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(RevenueCategory),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(RevenueCategory),
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
    pub validation_error: Option<String>,
}

impl EditState {
    pub fn new(revenue_category: &RevenueCategory) -> Self {
        Self {
            name: revenue_category.name.clone(),
            id: revenue_category.id.to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Revenue category name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Revenue category ID must be between 1 and 999".to_string()
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
pub struct RevenueCategory {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for RevenueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for RevenueCategory {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
        }
    }
}

impl RevenueCategory {

    pub fn new_draft() -> Self {
        Self {
            id: -1,  // Temporary UI-only ID
            name: String::new(),
            ..RevenueCategory::default()
        } 
    }

    fn validate(&self, other_categories: &[&RevenueCategory]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Revenue category ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_categories {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Revenue category with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Revenue category name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    revenue_category: &mut RevenueCategory,
    message: Message,
    state: &mut EditState,
    other_categories: &[&RevenueCategory]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                revenue_category.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    revenue_category.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if revenue_category.validate(other_categories).is_ok() {
                    Action::operation(Operation::Save(revenue_category.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(revenue_category.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_revenue_category = RevenueCategory::default();
            Action::operation(Operation::CreateNew(new_revenue_category))
        },
        Message::Select => {
            Action::operation(Operation::Select(revenue_category.id))
        },
    }
}

pub fn view<'a>(
    revenue_category: &'a RevenueCategory, 
    mode: &'a Mode,
    all_categories: &'a HashMap<EntityId, RevenueCategory>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(revenue_category).map(Message::View),
        Mode::Edit => {
            edit::view(
                revenue_category,
                EditState::new(revenue_category),
                all_categories
            ).map(Message::Edit)
        }
    }
}