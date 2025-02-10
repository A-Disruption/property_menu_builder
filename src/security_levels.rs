pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use iced::Element;
use iced::widget::{button, container, column, row, text};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(SecurityLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(SecurityLevel),
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
    pub fn new(security_level: &SecurityLevel) -> Self {
        Self {
            name: security_level.name.clone(),
            id: security_level.id.to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Security level name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Security level ID must be between 1 and 999".to_string()
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
pub struct SecurityLevel {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl SecurityLevel {

    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_levels: &[&SecurityLevel]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Security level ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Security level with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Security level name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    security_level: &mut SecurityLevel,
    message: Message,
    state: &mut EditState,
    other_levels: &[&SecurityLevel]
) -> Action<Operation, Message> {
    match message {
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                security_level.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    if security_level.id < 0 {
                        security_level.id = id;
                    }
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if security_level.validate(other_levels).is_ok() {
                    Action::operation(Operation::Save(security_level.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(security_level.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
        Message::CreateNew => {
            println!("CreateNew message received in security_levels");
            let new_security_level = SecurityLevel::default();
            Action::operation(Operation::CreateNew(new_security_level))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    security_level: &'a SecurityLevel, 
    mode: &'a Mode,
    all_levels: &'a HashMap<EntityId, SecurityLevel>
) -> Element<'a, Message> {
    let levels_list = column(
        all_levels
            .values()
            .map(|level| {
                button(text(&level.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(level.id))
                    .style(if level.id == security_level.id {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(200.0));

    let content = match mode {
        Mode::View => view::view(security_level).map(Message::View),
        Mode::Edit => {
            edit::view(
                security_level,
                EditState::new(security_level),
                all_levels
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                text("Security Levels").size(24),
                button("Create New")
                    .on_press(Message::CreateNew)
                    .style(button::primary),
                    levels_list,
            ]
            .spacing(10)
            .padding(10)
        )
        .style(container::rounded_box),
        container(content)
            .width(iced::Length::Fill)
            .style(container::rounded_box)
    ]
    .spacing(20)
    .into()
}
