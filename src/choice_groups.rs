use crate::data_types::{EntityId, ValidationError};
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use serde::{Serialize, Deserialize};
use iced::Element;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    SaveAll(EntityId, EditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    SaveAll(EntityId, EditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
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

impl Entity for ChoiceGroup {
    fn id(&self) -> EntityId {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn with_id(&self, id: EntityId) -> Self {
        let mut clone = self.clone();
        clone.id = id;
        clone
    }
    
    fn with_name(&self, name: String) -> Self {
        let mut clone = self.clone();
        clone.name = name;
        clone
    }
    
    fn default_new() -> Self {
        Self::default()
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
    message: Message,
) -> Action<Operation, Message> {
    match message {
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyChoiceGroup(id) => {
            Action::operation(Operation::CopyChoiceGroup(id))
        },
        Message::EditChoiceGroup(id) => {
            Action::operation(Operation::EditChoiceGroup(id))
        },
        Message::CreateNew => {
            Action::operation(Operation::CreateNew)
        },
        Message::SaveAll(id, edit_state) => {
            Action::operation(Operation::SaveAll(id, edit_state))
        }
        Message::UpdateName(id, new_name) => {
            Action::operation(Operation::UpdateName(id, new_name))
        }
        Message::CancelEdit(id) => {
            Action::operation(Operation::CancelEdit(id))
        }
    }
}

pub fn view<'a>(
    all_groups: &'a BTreeMap<EntityId, ChoiceGroup>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {
    entity_component::entity_view(
        "Choice Groups",
        Message::CreateNew,
        all_groups,
        edit_states,
        |choice_group, edit_states| render_choice_group_row(choice_group, edit_states)
    )
}

fn render_choice_group_row<'a>(
    choice_group: &'a ChoiceGroup,
    edit_states: &'a Vec<EditState>
) -> Element<'a, Message> {
    entity_component::entity_quick_edit_view(
        choice_group,
        edit_states,
        Message::EditChoiceGroup,
        Message::SaveAll,
        Message::CopyChoiceGroup,
        Message::RequestDelete,
        Message::CancelEdit,
        Message::UpdateName,
        "Choice Group Name"
    )
}

fn get_next_id(groups: &BTreeMap<EntityId, ChoiceGroup>) -> EntityId {
    groups
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
}