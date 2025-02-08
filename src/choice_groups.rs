pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
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
    Save(ChoiceGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ChoiceGroup),
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
    pub fn new(choice_group: &ChoiceGroup) -> Self {
        Self {
            name: choice_group.name.clone(),
            id: choice_group.id.to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice group name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Choice group ID must be between 1 and 999".to_string()
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

#[derive(Debug, Clone)]
pub enum ValidationError {
    InvalidId(String),
    DuplicateId(String),
    EmptyName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceGroup {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ChoiceGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for ChoiceGroup {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::new(),
        }
    }
}

impl ChoiceGroup {

    pub fn new_draft() -> Self {
        Self {
            id: -1,  // Temporary UI-only ID
            name: String::new(),
            ..ChoiceGroup::default()
        } 
    }

    fn validate(&self, other_groups: &[&ChoiceGroup]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Choice group ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_groups {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Choice group with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Choice group name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    choice_group: &mut ChoiceGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&ChoiceGroup]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                choice_group.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    choice_group.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if choice_group.validate(other_groups).is_ok() {
                    Action::operation(Operation::Save(choice_group.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(choice_group.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            let new_choice_group = ChoiceGroup::default();
            Action::operation(Operation::CreateNew(new_choice_group))
        },
        Message::Select => {
            Action::operation(Operation::Select(choice_group.id))
        },
    }
}

pub fn view<'a>(
    choice_group: &'a ChoiceGroup, 
    mode: &'a Mode,
    all_groups: &'a HashMap<EntityId, ChoiceGroup>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(choice_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                choice_group,
                EditState::new(choice_group),
                all_groups
            ).map(Message::Edit)
        }
    }
}