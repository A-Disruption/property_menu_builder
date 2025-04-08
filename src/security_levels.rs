use crate::data_types::{
    self,
    EntityId,
    ValidationError,
    Validatable,
};
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{button, container, column, row, text, text_input, scrollable};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopySecurityLevel(EntityId),
    EditSecurityLevel(EntityId),
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
    Save(SecurityLevel),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(SecurityLevel),
    RequestDelete(EntityId),
    CopySecurityLevel(EntityId),
    EditSecurityLevel(EntityId),
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

impl Entity for SecurityLevel {
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

impl SecurityLevel {
    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_levels: &[&SecurityLevel]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Price Levels ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_levels {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Price Levels with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Price Levels name cannot be empty".to_string()
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
        Message::CreateNew => {
            let new_security_level = SecurityLevel::default();
            Action::operation(Operation::CreateNew(new_security_level))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopySecurityLevel(id) => {
            Action::operation(Operation::CopySecurityLevel(id))
        },
        Message::EditSecurityLevel(id) => {
            Action::operation(Operation::EditSecurityLevel(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                security_level.id = id;
                Action::none()
            } else {
                state.id_validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            security_level.name = name;
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
    all_levels: &'a BTreeMap<EntityId, SecurityLevel>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {
    entity_component::entity_view(
        "Security Levels",
        Message::CreateNewMulti,
        all_levels,
        edit_states,
        |security_level, edit_states| render_security_level_row(security_level, edit_states),
    )
}

fn render_security_level_row<'a>(
    security_level: &'a SecurityLevel,
    edit_states: &'a Vec<EditState>
) -> Element<'a, Message> {
    entity_component::entity_quick_edit_view(
        security_level,
        edit_states,
        Message::EditSecurityLevel,
        Message::SaveAll,
        Message::CopySecurityLevel,
        Message::RequestDelete,
        Message::CancelEdit,
        Message::UpdateMultiName,
        "Security Level Name"
    )
}

fn get_next_id(levels: &BTreeMap<EntityId, SecurityLevel>) -> EntityId {
    levels
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
}