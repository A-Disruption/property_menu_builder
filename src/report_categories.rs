use crate::data_types::{EntityId, ValidationError };
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use serde::{Serialize, Deserialize};
use iced::Element;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    RequestDelete(EntityId),
    CopyReportCategory(EntityId),
    EditReportCategory(EntityId),
    SaveAll(EntityId, EditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone)]
pub enum Operation {
    RequestDelete(EntityId),
    CopyReportCategory(EntityId),
    EditReportCategory(EntityId),
    SaveAll(EntityId, EditState),
    UpdateName(EntityId, String),
    CreateNew,
    CancelEdit(EntityId),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            id: -1,
            name: String::new(),
        }
    }
}

impl Entity for ReportCategory {
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

impl ReportCategory {
    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_printers: &[&ReportCategory]) -> Result<(), ValidationError> {
        // Validate ID range (0-25 based on your screenshot)
        if !(0..=25).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Report Category ID must be between 0 and 25".to_string()
            ));
        }

        // Check for duplicate IDs
        for other in other_printers {
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

        // Validate name is not more than 16 Characters
        if self.name.len() > 16 {
            return Err(ValidationError::NameTooLong(
                "Report Category name cannot be more than 16 Characters".to_string()
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
        Message::CopyReportCategory(id) => {
            Action::operation(Operation::CopyReportCategory(id))
        },
        Message::EditReportCategory(id) => {
            println!("Editing ID: {}", id);
            Action::operation(Operation::EditReportCategory(id))
        }
        Message::CreateNew => {
            Action::operation(Operation::CreateNew)
        }
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
    all_categories: &'a BTreeMap<EntityId, ReportCategory>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {
    entity_component::entity_view(
        "Report Categories",
        Message::CreateNew,
        all_categories,
        edit_states,
        |category, edit_states| render_category_row(category, edit_states),
    )
}

fn render_category_row<'a>(
    category: &'a ReportCategory,
    edit_states: &'a Vec<EditState>
) -> Element<'a, Message> {
    entity_component::entity_quick_edit_view(
        category,
        edit_states,
        Message::EditReportCategory,
        Message::SaveAll,
        Message::CopyReportCategory,
        Message::RequestDelete,
        Message::CancelEdit,
        Message::UpdateName,
        "Report Category Name"
    )
}