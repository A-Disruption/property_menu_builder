use crate::data_types::{
    self,
    EntityId,
    Validatable,
    ValidationError,
};
use crate::Action;
use crate::entity_component::{self, Entity, EditState};
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::{Element, Length};
use iced::widget::{column, container, row, text, button, text_input, scrollable};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    CreateNew,
    RequestDelete(EntityId),
    CopyProductClass(EntityId),
    EditProductClass(EntityId),
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
    Save(ProductClass),
    StartEdit(EntityId),
    Cancel,
    Back,
    CreateNew(ProductClass),
    RequestDelete(EntityId),
    CopyProductClass(EntityId),
    EditProductClass(EntityId),
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
pub struct ProductClass {
    pub id: EntityId,
    pub name: String,
}

impl std::fmt::Display for ProductClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for ProductClass {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
        }
    }
}

impl Entity for ProductClass {
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

impl ProductClass {
    pub fn new_draft() -> Self {
        Self::default()
    }

    fn validate(&self, other_classes: &[&ProductClass]) -> Result<(), ValidationError> {
        if !(1..=999).contains(&self.id) {
            return Err(ValidationError::InvalidId(
                "Product class ID must be between 1 and 999".to_string()
            ));
        }

        for other in other_classes {
            if other.id == self.id {
                return Err(ValidationError::DuplicateId(
                    format!("Product class with ID {} already exists", self.id)
                ));
            }
        }

        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Product class name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

pub fn update(
    product_class: &mut ProductClass,
    message: Message,
    state: &mut EditState,
    other_classes: &[&ProductClass]
) -> Action<Operation, Message> {
    match message {
        Message::CreateNew => {
            let new_product_class = ProductClass::default();
            Action::operation(Operation::CreateNew(new_product_class))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyProductClass(id) => {
            Action::operation(Operation::CopyProductClass(id))
        },
        Message::EditProductClass(id) => {
            Action::operation(Operation::EditProductClass(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
        Message::UpdateId(id) => {
            if let Ok(id) = id.parse() {
                product_class.id = id;
                Action::none()
            } else {
                state.id_validation_error = Some("Invalid ID format".to_string());
                Action::none()
            }
        },
        Message::UpdateName(name) => {
            product_class.name = name;
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
    all_groups: &'a BTreeMap<EntityId, ProductClass>,
    edit_states: &'a Vec<EditState>,
) -> Element<'a, Message> {
    entity_component::entity_view(
        "Product classes",
        Message::CreateNewMulti,
        all_groups,
        edit_states,
        |product_class, edit_states| render_product_class_row(product_class, edit_states),
    )
}

fn render_product_class_row<'a>(
    product_class: &'a ProductClass,
    edit_states: &'a Vec<EditState>
) -> Element<'a, Message> {
    entity_component::entity_quick_edit_view(
        product_class,
        edit_states,
        Message::EditProductClass,
        Message::SaveAll,
        Message::CopyProductClass,
        Message::RequestDelete,
        Message::CancelEdit,
        Message::UpdateMultiName,
        "Product class Name"
    )
}

fn get_next_id(classes: &BTreeMap<EntityId, ProductClass>) -> EntityId {
    classes
        .keys()
        .max()
        .map_or(1, |max_id| max_id + 1)
}