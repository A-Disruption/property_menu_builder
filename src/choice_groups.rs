use crate::data_types::{self, EntityId, ValidationError};
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::Element;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    UpdateId(String),
    UpdateName(String),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Save(ChoiceGroup),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ChoiceGroup),
    RequestDelete(EntityId),
    CopyChoiceGroup(EntityId),
    EditChoiceGroup(EntityId),
    Select(EntityId),
    SaveAll(EntityId, EditState),
    UpdateMultiName(EntityId, String),
    CreateNewMulti,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Mode {
    View,
    Edit,
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
    choice_group: &mut ChoiceGroup,
    message: Message,
    state: &mut EditState,
    other_groups: &[&ChoiceGroup]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            let new_choice_group = ChoiceGroup::default();
            Action::operation(Operation::CreateNew(new_choice_group))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyChoiceGroup(id) => {
            Action::operation(Operation::CopyChoiceGroup(id))
        },
        Message::EditChoiceGroup(id) => {
            Action::operation(Operation::EditChoiceGroup(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                choice_group.id = id;
                Action::none()
            } else {
                state.id_validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            choice_group.name = name;
            Action::none()
        },
        Message::CreateNewMulti => {
            Action::operation(Operation::CreateNewMulti)
        },
        Message::SaveAll(id, edit_state) => {
            Action::operation(Operation::SaveAll(id, edit_state))
        }
        Message::UpdateMultiName(id, new_name) => {
            Action::operation(Operation::UpdateMultiName(id, new_name))
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
        Message::CreateNewMulti,
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
        Message::UpdateMultiName,
        "Choice Group Name"
    )
}

fn get_next_id(groups: &BTreeMap<EntityId, ChoiceGroup>) -> EntityId {
    groups
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
}