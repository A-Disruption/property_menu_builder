pub mod edit;
pub mod view;

use crate::data_types::{
    EntityId,
    Validatable,
    ValidationError,
};
use crate::Action;
use crate::icon;
use serde::{Serialize, Deserialize};
use iced::Element;
use iced::widget::{column, container, row, text, button};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Message {
    Edit(edit::Message),
    View(view::Message),
    CreateNew,
    RequestDelete(EntityId),
    CopyProductClass(EntityId),
    Select(EntityId),
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
        Message::Edit(msg) => match msg {
            edit::Message::UpdateName(name) => {
                product_class.name = name;
                Action::none()
            }
            edit::Message::UpdateId(id) => {
                if let Ok(id) = id.parse() {
                    if product_class.id < 0 {
                        product_class.id = id;
                    }
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
        Message::CreateNew => {
            let new_product_classes = ProductClass::default();
            Action::operation(Operation::CreateNew(new_product_classes))
        },
        Message::RequestDelete(id) => {
            Action::operation(Operation::RequestDelete(id))
        },
        Message::CopyProductClass(id) => {
            Action::operation(Operation::CopyProductClass(id))
        },
        Message::Select(id) => {
            Action::operation(Operation::Select(id))
        },
    }
}

pub fn view<'a>(
    product_class: &'a ProductClass, 
    mode: &'a Mode,
    all_classes: &'a BTreeMap<EntityId, ProductClass>
) -> Element<'a, Message> {

    let classes_list = column(
        all_classes
            .values()
            .map(|class| {
                button(
                    list_item(
                        &class.name.as_str(), 
                        button(icon::copy())
                            .on_press(Message::CopyProductClass(class.id))
                            .style(
                                if class.id == product_class.id {
                                    button::secondary
                                } else {
                                    button::primary
                                }
                            ), 
                        button(icon::trash()).on_press(Message::RequestDelete(class.id)),
                    )
                )
                .width(iced::Length::Fill)
                .on_press(Message::Select(class.id))
                .style(if class.id == product_class.id {
                    button::primary
                } else {
                    button::secondary
                })
                .into()
            })
            .collect::<Vec<_>>()
    )
    .spacing(5)
    .width(iced::Length::Fixed(250.0));

    let content = match mode {
        Mode::View => view::view(product_class).map(Message::View),
        Mode::Edit => {
            edit::view(
                product_class,
                EditState::new(product_class),
                all_classes
            ).map(Message::Edit)
        }
    };

    row![
        container(
            column![
                row![
                    text("Product Classes").size(18),
                    iced::widget::horizontal_space(),
                    button(icon::new().shaping(text::Shaping::Advanced))
                        .on_press(Message::CreateNew)
                        .style(button::primary),
                ].width(250),
                classes_list,
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


pub fn list_item<'a>(list_text: &'a str, copy_button: iced::widget::Button<'a, Message>,delete_button: iced::widget::Button<'a, Message>) -> Element<'a, Message> {
    let button_content = container (
        row![
            text(list_text),
            iced::widget::horizontal_space(),
            copy_button,
            delete_button.style(button::danger)
        ].align_y(iced::Alignment::Center),
    );
    
    button_content.into()
}