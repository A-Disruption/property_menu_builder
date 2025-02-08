pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    IdRange,
    ValidationError,
};
use crate::item_groups::{self, ItemGroup};
use crate::revenue_categories::RevenueCategory;
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
    Save(ProductClass),
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
    pub fn new(product_class: &ProductClass) -> Self {
        Self {
            name: product_class.name.clone(),
            id: product_class.id.to_string(),
            validation_error: None,
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName(
                "Product class name cannot be empty".to_string()
            ));
        }

        if let Ok(id) = self.id.parse::<EntityId>() {
            if !(1..=999).contains(&id) {
                return Err(ValidationError::InvalidId(
                    "Product class ID must be between 1 and 999".to_string()
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

/* pub struct UpdateContext<'a> {
    pub other_classes: &'a [&'a ProductClass],
    pub available_item_groups: &'a [&'a ItemGroup],
    pub available_revenue_categories: &'a [&'a RevenueCategory],
}
 */


#[derive(Debug, Clone, PartialEq)]
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
            id: 1,
            name: String::new(),
        }
    }
}

impl ProductClass {
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
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                product_class.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    product_class.id = id;
                    Action::none()
                } else {
                    state.validation_error = Some("Invalid ID format".to_string());
                    Action::none()
                }
            }
            edit::Message::Save => {
                if product_class.validate(other_classes).is_ok() {
                    Action::operation(Operation::Save(product_class.clone()))
                } else {
                    state.validation_error = Some("Validation failed".to_string());
                    Action::none()
                }
            }
            edit::Message::Cancel => Action::operation(Operation::Cancel),
        },
        Message::View(msg) => match msg {
            view::Message::Edit => Action::operation(Operation::StartEdit(product_class.id)),
            view::Message::Back => Action::operation(Operation::Back),
        }
    }
}

pub fn view<'a>(
    product_class: &'a ProductClass, 
    mode: &'a Mode,
    all_classes: &'a HashMap<EntityId, ProductClass>
) -> Element<'a, Message> {
    match mode {
        Mode::View => view::view(product_class).map(Message::View),
        Mode::Edit => {
            edit::view(
                product_class,
                EditState::new(product_class),
                all_classes
            ).map(Message::Edit)
        }
    }
}