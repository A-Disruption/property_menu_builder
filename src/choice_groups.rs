pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::widget::{button, row, column, container, text};
use iced::Element;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    RequestDelete(EntityId),
    ConfirmDelete(EntityId),
    CancelDelete,
    Select(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ChoiceGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ChoiceGroup),
    RequestDelete(EntityId),
    ConfirmDelete(EntityId),
    CancelDelete,
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
    pub next_id: EntityId,
}

impl EditState {
    pub fn new(choice_group: &ChoiceGroup, next_id: EntityId) -> Self {
        Self {
            name: choice_group.name.clone(),
            id: choice_group.id.to_string(),
            validation_error: None,
            next_id,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            id: -1,
            name: String::new(),
        }
    }
}

impl ChoiceGroup {

    pub fn new_draft() -> Self {
        Self::default()
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
                    //Only allow ID updates for new items
                    if choice_group.id < 0 {
                        choice_group.id = id;
                    }
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
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::ConfirmDelete(id) => {
            Action::operation(Operation::ConfirmDelete(id))
        },
        Message::CancelDelete => {
            Action::operation(Operation::CancelDelete)
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    choice_group: &'a ChoiceGroup, 
    mode: &'a Mode,
    all_groups: &'a BTreeMap<EntityId, ChoiceGroup>
) -> Element<'a, Message> {

    let groups_list = column(
        all_groups
            .values()
            .map(|group| {
                button(text(&group.name))
                    .width(iced::Length::Fill)
                    .on_press(Message::Select(group.id))
                    .style(if group.id == choice_group.id {
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
        Mode::View => view::view(choice_group).map(Message::View),
        Mode::Edit => {
            edit::view(
                choice_group,
                EditState::new(choice_group, get_next_id(all_groups)),
                all_groups
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                row![
                    text("Choice Groups").size(18),
                    iced::widget::horizontal_space(),
                    button(icon::new().shaping(text::Shaping::Advanced))
                        .on_press(Message::CreateNew)
                        .style(button::primary),
                ].width(200),
                groups_list,
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

fn get_next_id(groups: &BTreeMap<EntityId, ChoiceGroup>) -> EntityId {
    groups
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
}